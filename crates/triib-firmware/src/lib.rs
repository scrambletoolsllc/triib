//! What a firmware file is, for triib and triib-cli as they send one to
//! a device's FIRMWARE_IMAGE memory object: an ESP-IDF application
//! image's chip, project, version, ELF hash and signatures, whether it
//! suits the memory object, and whether the device runs it afterwards.

pub mod esp_image;
