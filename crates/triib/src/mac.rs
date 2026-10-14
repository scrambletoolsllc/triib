//! What macOS makes of an AVB entity: the Core Audio device apps play
//! into and record from, as it makes one of the Mac's own entity, and a
//! tap on what apps send through it.

#![allow(unsafe_code)]

use std::ptr::NonNull;

use atdecc::EntityId;
use objc2_core_audio::{
    AudioObjectGetPropertyData, AudioObjectGetPropertyDataSize, AudioObjectID,
    AudioObjectPropertyAddress, AudioObjectPropertySelector, kAudioDevicePropertyDeviceUID,
    kAudioDevicePropertyStreamConfiguration, kAudioHardwarePropertyDevices,
    kAudioObjectPropertyElementMain, kAudioObjectPropertyName, kAudioObjectPropertyScopeGlobal,
    kAudioObjectPropertyScopeInput, kAudioObjectPropertyScopeOutput, kAudioObjectSystemObject,
};
use objc2_core_foundation::{CFRetained, CFString};

fn address(selector: AudioObjectPropertySelector) -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: selector,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMain,
    }
}

/// Every Core Audio device.
fn devices() -> Vec<AudioObjectID> {
    let system = kAudioObjectSystemObject as AudioObjectID;
    let mut address = address(kAudioHardwarePropertyDevices);
    let mut size = 0u32;
    // SAFETY: an address and a size to write to.
    let status = unsafe {
        AudioObjectGetPropertyDataSize(
            system,
            NonNull::from(&mut address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
        )
    };
    if status != 0 {
        return Vec::new();
    }
    let mut devices: Vec<AudioObjectID> =
        vec![0; size as usize / std::mem::size_of::<AudioObjectID>()];
    // SAFETY: room for `size` octets of device IDs.
    let status = unsafe {
        AudioObjectGetPropertyData(
            system,
            NonNull::from(&mut address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
            NonNull::new_unchecked(devices.as_mut_ptr().cast()),
        )
    };
    if status != 0 {
        return Vec::new();
    }
    devices.truncate(size as usize / std::mem::size_of::<AudioObjectID>());
    devices
}

/// A device's text property, such as its name or UID.
fn text(device: AudioObjectID, selector: AudioObjectPropertySelector) -> Option<String> {
    let mut address = address(selector);
    let mut value: *const CFString = std::ptr::null();
    let mut size = std::mem::size_of::<*const CFString>() as u32;
    // SAFETY: room for the one CFStringRef the property gives, retained.
    let status = unsafe {
        AudioObjectGetPropertyData(
            device,
            NonNull::from(&mut address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
            NonNull::from(&mut value).cast(),
        )
    };
    if status != 0 {
        return None;
    }
    // SAFETY: the property handed over a CFString it retained.
    let value = unsafe { CFRetained::from_raw(NonNull::new(value.cast_mut())?) };
    Some(value.to_string())
}

/// The name of the Core Audio device macOS made of AVB entity
/// `entity_id`, whose UID carries the entity's ID, as
/// "AAA:0xd111e597f5448000:0x0000:0x0000" does.
pub fn audio_device_of(entity_id: EntityId) -> Option<String> {
    let id = format!(":{entity_id}:").to_lowercase();
    devices()
        .into_iter()
        .find(|&device| {
            text(device, kAudioDevicePropertyDeviceUID)
                .is_some_and(|uid| uid.to_lowercase().contains(&id))
        })
        .and_then(|device| text(device, kAudioObjectPropertyName))
}

/// The device with `name`.
fn device_named(name: &str) -> Option<AudioObjectID> {
    devices()
        .into_iter()
        .find(|&device| text(device, kAudioObjectPropertyName).as_deref() == Some(name))
}

/// Each of a device's streams' channels on one side, from its stream
/// configuration: an AudioBufferList with a buffer for each stream.
fn stream_channels_of(device: AudioObjectID, input: bool) -> Vec<u16> {
    let mut address = AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyStreamConfiguration,
        mScope: if input {
            kAudioObjectPropertyScopeInput
        } else {
            kAudioObjectPropertyScopeOutput
        },
        mElement: kAudioObjectPropertyElementMain,
    };
    let mut size = 0u32;
    // SAFETY: an address and a size to write to.
    let status = unsafe {
        AudioObjectGetPropertyDataSize(
            device,
            NonNull::from(&mut address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
        )
    };
    if status != 0 || size < 4 {
        return Vec::new();
    }
    // Eight-byte words, as the list holds pointers.
    let mut list = vec![0u64; (size as usize).div_ceil(8)];
    // SAFETY: room for `size` octets, aligned for the list.
    let status = unsafe {
        AudioObjectGetPropertyData(
            device,
            NonNull::from(&mut address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
            NonNull::new_unchecked(list.as_mut_ptr().cast()),
        )
    };
    if status != 0 {
        return Vec::new();
    }
    let octets: Vec<u8> = list.iter().flat_map(|word| word.to_ne_bytes()).collect();
    let word = |at: usize| {
        octets
            .get(at..at + 4)
            .map(|bytes| u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    };
    // mNumberBuffers, then from offset 8 each 16-octet AudioBuffer, its
    // mNumberChannels first.
    let count = word(0).unwrap_or(0) as usize;
    (0..count)
        .filter_map(|buffer| word(8 + buffer * 16))
        .map(|channels| channels as u16)
        .collect()
}

/// The channels of each of the output streams of the device with `name`,
/// whose audio a [`SentTap`] can hear.
pub fn output_streams(name: &str) -> Vec<u16> {
    device_named(name).map_or_else(Vec::new, |device| stream_channels_of(device, false))
}

/// What every app sends to one output stream of a device, heard through a
/// Core Audio process tap, and delivered as the input of a private
/// aggregate device around it, which the tapped device clocks: its own
/// inputs first, then the tap's channels. Both go when it is dropped.
pub struct SentTap {
    tap: AudioObjectID,
    aggregate: AudioObjectID,
    /// The aggregate device's name, to open it by.
    pub name: String,
    /// The aggregate's input channels before the tap's.
    pub offset: u16,
    /// The tap's channels: the stream's.
    pub channels: u16,
}

impl SentTap {
    /// Taps output stream `stream` of the device named `device`.
    pub fn open(device: &str, stream: u16) -> Result<Self, String> {
        use objc2::AnyThread;
        use objc2::rc::Retained;
        use objc2_core_audio::{
            AudioHardwareCreateAggregateDevice, AudioHardwareCreateProcessTap,
            AudioHardwareDestroyProcessTap, CATapDescription, CATapMuteBehavior,
            kAudioAggregateDeviceIsPrivateKey, kAudioAggregateDeviceMainSubDeviceKey,
            kAudioAggregateDeviceNameKey, kAudioAggregateDeviceSubDeviceListKey,
            kAudioAggregateDeviceTapAutoStartKey, kAudioAggregateDeviceTapListKey,
            kAudioAggregateDeviceUIDKey, kAudioSubDeviceUIDKey, kAudioSubTapDriftCompensationKey,
            kAudioSubTapUIDKey,
        };
        use objc2_core_foundation::CFDictionary;
        use objc2_foundation::{NSArray, NSDictionary, NSNumber, NSObject, NSString};

        let tapped =
            device_named(device).ok_or_else(|| format!("no audio device named {device}"))?;
        let uid = text(tapped, kAudioDevicePropertyDeviceUID).ok_or("the device has no UID")?;
        // SAFETY: CATapDescription's initializer and setters, with the
        // types its header declares.
        let description = unsafe {
            let description = CATapDescription::initExcludingProcesses_andDeviceUID_withStream(
                CATapDescription::alloc(),
                &NSArray::new(),
                &NSString::from_str(&uid),
                stream as isize,
            );
            description.setName(&NSString::from_str("triib listening"));
            description.setPrivate(true);
            description.setMuteBehavior(CATapMuteBehavior::Unmuted);
            description
        };
        let mut tap: AudioObjectID = 0;
        // SAFETY: a description and a tap ID to write to.
        let status = unsafe { AudioHardwareCreateProcessTap(Some(&description), &mut tap) };
        if status != 0 {
            return Err(format!("Core Audio refused the tap ({status})"));
        }
        // SAFETY: the description's UUID, a plain getter.
        let tap_uid = unsafe { description.UUID() }.UUIDString().to_string();

        let key = |key: &std::ffi::CStr| NSString::from_str(&key.to_string_lossy());
        let string = |text: &str| Retained::into_super(NSString::from_str(text));
        let number =
            |value: i32| Retained::into_super(Retained::into_super(NSNumber::new_i32(value)));
        let dictionary = |entries: Vec<(Retained<NSString>, Retained<NSObject>)>| {
            let keys: Vec<&NSString> = entries.iter().map(|(key, _)| &**key).collect();
            let values: Vec<Retained<NSObject>> =
                entries.iter().map(|(_, value)| value.clone()).collect();
            NSDictionary::from_retained_objects(&keys, &values)
        };
        let array = |items: Vec<Retained<NSDictionary<NSString, NSObject>>>| {
            Retained::into_super(NSArray::from_retained_slice(&items))
        };
        let id = std::process::id();
        let name = format!("triib listening {id}");
        let description = dictionary(vec![
            (key(kAudioAggregateDeviceNameKey), string(&name)),
            (
                key(kAudioAggregateDeviceUIDKey),
                string(&format!("io.github.scrambletools.triib.listen.{id}")),
            ),
            (key(kAudioAggregateDeviceMainSubDeviceKey), string(&uid)),
            (key(kAudioAggregateDeviceIsPrivateKey), number(1)),
            (key(kAudioAggregateDeviceTapAutoStartKey), number(1)),
            (
                key(kAudioAggregateDeviceSubDeviceListKey),
                array(vec![dictionary(vec![(
                    key(kAudioSubDeviceUIDKey),
                    string(&uid),
                )])]),
            ),
            (
                key(kAudioAggregateDeviceTapListKey),
                array(vec![dictionary(vec![
                    (key(kAudioSubTapUIDKey), string(&tap_uid)),
                    (key(kAudioSubTapDriftCompensationKey), number(1)),
                ])]),
            ),
        ]);
        let mut aggregate: AudioObjectID = 0;
        // SAFETY: an NSDictionary is a CFDictionary, toll-free bridged.
        let status = unsafe {
            let description = &*(Retained::as_ptr(&description).cast::<CFDictionary>());
            AudioHardwareCreateAggregateDevice(description, NonNull::from(&mut aggregate))
        };
        if status != 0 {
            // SAFETY: the tap made above.
            unsafe { AudioHardwareDestroyProcessTap(tap) };
            return Err(format!(
                "Core Audio refused the aggregate device ({status})"
            ));
        }
        let offset: u16 = stream_channels_of(tapped, true).iter().sum();
        let total: u16 = stream_channels_of(aggregate, true).iter().sum();
        Ok(Self {
            tap,
            aggregate,
            name,
            offset,
            channels: total.saturating_sub(offset),
        })
    }
}

impl Drop for SentTap {
    fn drop(&mut self) {
        use objc2_core_audio::{
            AudioHardwareDestroyAggregateDevice, AudioHardwareDestroyProcessTap,
        };
        // SAFETY: the aggregate device and the tap this made.
        unsafe {
            AudioHardwareDestroyAggregateDevice(self.aggregate);
            AudioHardwareDestroyProcessTap(self.tap);
        }
    }
}
