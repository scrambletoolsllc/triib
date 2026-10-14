//! What macOS makes of an AVB entity: the Core Audio device apps play
//! into and record from, as it makes one of the Mac's own entity, and a
//! tap on what apps send through it.

#![allow(unsafe_code)]

use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::{Arc, OnceLock};

use atdecc::EntityId;
use objc2_core_audio::{
    AudioObjectGetPropertyData, AudioObjectGetPropertyDataSize, AudioObjectID,
    AudioObjectPropertyAddress, AudioObjectPropertySelector, kAudioDevicePropertyDeviceUID,
    kAudioDevicePropertyStreamConfiguration, kAudioHardwarePropertyDevices,
    kAudioObjectPropertyElementMain, kAudioObjectPropertyName, kAudioObjectPropertyScopeGlobal,
    kAudioObjectPropertyScopeInput, kAudioObjectPropertyScopeOutput, kAudioObjectSystemObject,
};
use objc2_core_audio_types::{AudioBuffer, AudioBufferList, AudioTimeStamp};
use objc2_core_foundation::{CFRetained, CFString};
use triib_stream::audio::{FrameBuffer, SharedBuffer};

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

/// A device's nominal sample rate.
fn rate_of(device: AudioObjectID) -> Option<f64> {
    use objc2_core_audio::kAudioDevicePropertyNominalSampleRate;
    let mut address = address(kAudioDevicePropertyNominalSampleRate);
    let mut rate = 0f64;
    let mut size = std::mem::size_of::<f64>() as u32;
    // SAFETY: room for the one Float64 the property gives.
    let status = unsafe {
        AudioObjectGetPropertyData(
            device,
            NonNull::from(&mut address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
            NonNull::from(&mut rate).cast(),
        )
    };
    (status == 0 && rate > 0.0).then_some(rate)
}

/// The channels of each of the output streams of the device with `name`,
/// whose audio a [`SentTap`] can hear.
pub fn output_streams(name: &str) -> Vec<u16> {
    device_named(name).map_or_else(Vec::new, |device| stream_channels_of(device, false))
}

/// Core Audio's process taps, which came with macOS 14.2. Their functions
/// are looked up when first used rather than linked, so that triib starts
/// on older systems, which lack them, and only taps are unavailable there.
struct Taps {
    create: unsafe extern "C" fn(*const c_void, *mut AudioObjectID) -> i32,
    destroy: unsafe extern "C" fn(AudioObjectID) -> i32,
}

fn taps() -> Option<&'static Taps> {
    static TAPS: OnceLock<Option<Taps>> = OnceLock::new();
    TAPS.get_or_init(|| {
        objc2::runtime::AnyClass::get(c"CATapDescription")?;
        // SAFETY: looking up symbols by name in what the process loaded.
        let (create, destroy) = unsafe {
            (
                libc::dlsym(
                    libc::RTLD_DEFAULT,
                    c"AudioHardwareCreateProcessTap".as_ptr(),
                ),
                libc::dlsym(
                    libc::RTLD_DEFAULT,
                    c"AudioHardwareDestroyProcessTap".as_ptr(),
                ),
            )
        };
        if create.is_null() || destroy.is_null() {
            return None;
        }
        // SAFETY: the two functions, as Core Audio's header declares them.
        Some(unsafe {
            Taps {
                create: std::mem::transmute::<
                    *mut c_void,
                    unsafe extern "C" fn(*const c_void, *mut AudioObjectID) -> i32,
                >(create),
                destroy: std::mem::transmute::<
                    *mut c_void,
                    unsafe extern "C" fn(AudioObjectID) -> i32,
                >(destroy),
            }
        })
    })
    .as_ref()
}

/// Whether this Mac can tap what apps send: macOS 14.2 and later.
pub fn taps_supported() -> bool {
    taps().is_some()
}

/// What every app sends to one output stream of a device, heard through a
/// Core Audio process tap, and delivered as the input of a private
/// aggregate device holding the tap alone, as Apple's own example has it:
/// its inputs are the tap's channels. The aggregate is read by an I/O
/// callback of its own, as audio libraries do not list a private device.
/// All of it goes when it is dropped.
pub struct SentTap {
    tap: AudioObjectID,
    aggregate: AudioObjectID,
    /// The tap's channels: the stream's.
    pub channels: u16,
    /// The callback reading the aggregate, once started, and what it
    /// fills.
    reading: Option<(objc2_core_audio::AudioDeviceIOProcID, NonNull<Reader>)>,
}

/// What the callback fills, and with which of the tap's channels.
struct Reader {
    buffer: Arc<FrameBuffer>,
    first: usize,
    wanted: usize,
    /// The frames of one callback, kept so as to allocate once.
    frames: Vec<f32>,
}

/// Copies channels `first..first + wanted` of the aggregate's input,
/// whatever its buffers, into the reader's buffer as interleaved frames.
unsafe extern "C-unwind" fn deliver(
    _device: AudioObjectID,
    _now: NonNull<AudioTimeStamp>,
    input: NonNull<AudioBufferList>,
    _input_time: NonNull<AudioTimeStamp>,
    _output: NonNull<AudioBufferList>,
    _output_time: NonNull<AudioTimeStamp>,
    reader: *mut c_void,
) -> i32 {
    // SAFETY: the reader the callback was made with, which only it
    // touches until it is stopped.
    let reader = unsafe { &mut *reader.cast::<Reader>() };
    // SAFETY: an AudioBufferList holds mNumberBuffers buffers from
    // mBuffers on, however many its declaration shows.
    let buffers: &[AudioBuffer] = unsafe {
        let list = input.as_ptr();
        std::slice::from_raw_parts(
            std::ptr::addr_of!((*list).mBuffers).cast::<AudioBuffer>(),
            (*list).mNumberBuffers as usize,
        )
    };
    let frames = buffers
        .iter()
        .filter(|buffer| buffer.mNumberChannels > 0 && !buffer.mData.is_null())
        .map(|buffer| buffer.mDataByteSize as usize / 4 / buffer.mNumberChannels as usize)
        .min()
        .unwrap_or(0);
    if frames == 0 {
        return 0;
    }
    let wanted = reader.wanted;
    reader.frames.clear();
    reader.frames.resize(frames * wanted, 0.0);
    let mut channel = 0;
    for buffer in buffers {
        let count = buffer.mNumberChannels as usize;
        if !buffer.mData.is_null() {
            // SAFETY: a buffer of at least `frames` frames of `count`
            // interleaved 32-bit floats, Core Audio's own format.
            let samples =
                unsafe { std::slice::from_raw_parts(buffer.mData.cast::<f32>(), frames * count) };
            for within in 0..count {
                let Some(slot) = (channel + within)
                    .checked_sub(reader.first)
                    .filter(|slot| *slot < wanted)
                else {
                    continue;
                };
                for frame in 0..frames {
                    reader.frames[frame * wanted + slot] = samples[frame * count + within];
                }
            }
        }
        channel += count;
    }
    reader.buffer.device_push(&reader.frames);
    0
}

impl SentTap {
    /// Taps output stream `stream` of the device named `device`.
    pub fn open(device: &str, stream: u16) -> Result<Self, String> {
        use objc2::AnyThread;
        use objc2::rc::Retained;
        use objc2_core_audio::{
            AudioHardwareCreateAggregateDevice, CATapDescription, CATapMuteBehavior,
            kAudioAggregateDeviceIsPrivateKey, kAudioAggregateDeviceNameKey,
            kAudioAggregateDeviceTapAutoStartKey, kAudioAggregateDeviceTapListKey,
            kAudioAggregateDeviceUIDKey, kAudioSubTapDriftCompensationKey, kAudioSubTapUIDKey,
        };
        use objc2_core_foundation::CFDictionary;
        use objc2_foundation::{NSArray, NSDictionary, NSNumber, NSObject, NSString};

        let taps = taps().ok_or("hearing what a device sends takes macOS 14.2 or later")?;
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
        let status = unsafe { (taps.create)(Retained::as_ptr(&description).cast(), &mut tap) };
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
            (key(kAudioAggregateDeviceIsPrivateKey), number(1)),
            (key(kAudioAggregateDeviceTapAutoStartKey), number(1)),
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
            unsafe { (taps.destroy)(tap) };
            return Err(format!(
                "Core Audio refused the aggregate device ({status})"
            ));
        }

        // The tap's channels come to the aggregate a moment after it is
        // made, and until then it is no input to open.
        let mut made = Self {
            tap,
            aggregate,
            channels: 0,
            reading: None,
        };
        let since = std::time::Instant::now();
        while since.elapsed() < std::time::Duration::from_secs(3) {
            made.channels = stream_channels_of(aggregate, true).iter().sum();
            if made.channels > 0 {
                return Ok(made);
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        Err("the tap brought no channels".to_owned())
    }

    /// Starts reading `channels` of the tap's channels from
    /// `first_channel` on, into the buffer this returns.
    pub fn start(&mut self, first_channel: u16, channels: u16) -> Result<SharedBuffer, String> {
        use objc2_core_audio::{
            AudioDeviceCreateIOProcID, AudioDeviceDestroyIOProcID, AudioDeviceIOProcID,
            AudioDeviceStart,
        };
        if self.reading.is_some() {
            return Err("already reading".to_owned());
        }
        let rate = rate_of(self.aggregate).ok_or("the tap has no rate")?;
        // 10 ms kept beyond the device's chunks, as for other inputs.
        let buffer = Arc::new(FrameBuffer::new(channels, rate as usize / 100));
        let reader = NonNull::from(Box::leak(Box::new(Reader {
            buffer: buffer.clone(),
            first: usize::from(first_channel),
            wanted: usize::from(channels),
            frames: Vec::with_capacity(8192 * usize::from(channels)),
        })));
        let mut proc: AudioDeviceIOProcID = None;
        // SAFETY: a callback of the declared type, the reader it is
        // given, kept until the callback is destroyed, and an ID to
        // write to.
        let status = unsafe {
            AudioDeviceCreateIOProcID(
                self.aggregate,
                Some(deliver),
                reader.as_ptr().cast(),
                NonNull::from(&mut proc),
            )
        };
        if status != 0 {
            // SAFETY: the reader leaked above, which nothing holds.
            drop(unsafe { Box::from_raw(reader.as_ptr()) });
            return Err(format!("Core Audio refused to read the tap ({status})"));
        }
        // SAFETY: the callback made above.
        let status = unsafe { AudioDeviceStart(self.aggregate, proc) };
        if status != 0 {
            // SAFETY: the callback made above, never started, and the
            // reader only it held.
            unsafe {
                AudioDeviceDestroyIOProcID(self.aggregate, proc);
                drop(Box::from_raw(reader.as_ptr()));
            }
            return Err(format!("Core Audio would not start the tap ({status})"));
        }
        self.reading = Some((proc, reader));
        Ok(SharedBuffer {
            buffer,
            channels,
            rate: rate.round() as u32,
        })
    }
}

impl Drop for SentTap {
    fn drop(&mut self) {
        use objc2_core_audio::{
            AudioDeviceDestroyIOProcID, AudioDeviceStop, AudioHardwareDestroyAggregateDevice,
        };
        // SAFETY: the callback, the aggregate device and the tap this
        // made; once the callback is stopped and destroyed, nothing holds
        // its reader.
        unsafe {
            if let Some((proc, reader)) = self.reading.take() {
                AudioDeviceStop(self.aggregate, proc);
                AudioDeviceDestroyIOProcID(self.aggregate, proc);
                drop(Box::from_raw(reader.as_ptr()));
            }
            AudioHardwareDestroyAggregateDevice(self.aggregate);
            // A tap was only made where taps are.
            if let Some(taps) = taps() {
                (taps.destroy)(self.tap);
            }
        }
    }
}
