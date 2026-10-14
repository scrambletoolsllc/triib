//! What macOS makes of an AVB entity: the Core Audio device apps play
//! into and record from, as it makes one of the Mac's own entity.

#![allow(unsafe_code)]

use std::ptr::NonNull;

use atdecc::EntityId;
use objc2_core_audio::{
    AudioObjectGetPropertyData, AudioObjectGetPropertyDataSize, AudioObjectID,
    AudioObjectPropertyAddress, AudioObjectPropertySelector, kAudioDevicePropertyDeviceUID,
    kAudioHardwarePropertyDevices, kAudioObjectPropertyElementMain, kAudioObjectPropertyName,
    kAudioObjectPropertyScopeGlobal, kAudioObjectSystemObject,
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
