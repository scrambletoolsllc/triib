//! ESP-IDF application images, as a firmware upload sends them to an
//! ESP32 board: what an image says of itself, whether it suits the
//! memory object it goes to, and, from a board's firmware_version before
//! and after, whether the board runs it.
//!
//! An image starts with ESP-IDF's header, magic 0xE9 and the chip's ID at
//! octet 12, then its esp_app_desc_t at octet 32: the project's version
//! and name and its ELF's SHA-256. A board's firmware_version ends with
//! the first nine hex digits of that SHA-256, `1.0.0 (esp_avb 2.45.0 -
//! dccd7ef2f)`, which tells the image it runs. A signed image ends in a
//! 4 KiB sector of Secure Boot V2 signature blocks.
//!
//! A coprocessor's firmware, such as ESP-Hosted's on an ESP32-C6 beside
//! an ESP32-P4, goes to a memory object of its own, named for it.

use std::fmt;

/// What an ESP-IDF application image says of itself.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EspImage {
    /// The chip it is built for, ESP-IDF's esp_chip_id_t.
    pub chip_id: u16,
    /// The project's version, CONFIG_APP_PROJECT_VER.
    pub version: String,
    /// The project's name.
    pub project: String,
    /// The SHA-256 of the ELF it was built from.
    pub elf_sha256: [u8; 32],
    /// Secure Boot V2 signature blocks in its last 4 KiB.
    pub signatures: usize,
}

/// Why octets are not an ESP-IDF application image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImageError {
    /// No 0xE9 header, or too short for one.
    NoHeader,
    /// No esp_app_desc_t where it goes.
    NoDescription,
}

impl fmt::Display for ImageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            ImageError::NoHeader => "not an ESP-IDF application image (no 0xE9 header)",
            ImageError::NoDescription => "not an ESP-IDF application image (no app description)",
        })
    }
}

impl std::error::Error for ImageError {}

/// Where esp_app_desc_t starts.
const DESCRIPTION: usize = 32;

/// Reads an ESP-IDF application image's header and description.
pub fn parse(image: &[u8]) -> Result<EspImage, ImageError> {
    if image.len() < DESCRIPTION + 208 || image[0] != 0xe9 {
        return Err(ImageError::NoHeader);
    }
    let word =
        |at: usize| u32::from_le_bytes([image[at], image[at + 1], image[at + 2], image[at + 3]]);
    if word(DESCRIPTION) != 0xabcd_5432 {
        return Err(ImageError::NoDescription);
    }
    let text = |at: usize, length: usize| {
        let field = &image[at..at + length];
        let end = field.iter().position(|&octet| octet == 0).unwrap_or(length);
        String::from_utf8_lossy(&field[..end]).into_owned()
    };
    let mut elf_sha256 = [0; 32];
    elf_sha256.copy_from_slice(&image[DESCRIPTION + 144..DESCRIPTION + 176]);
    let signatures = if image.len().is_multiple_of(4096) {
        let sector = &image[image.len() - 4096..];
        (0..3)
            .take_while(|block| sector[block * 1216..block * 1216 + 2] == [0xe7, 0x02])
            .count()
    } else {
        0
    };
    Ok(EspImage {
        chip_id: u16::from_le_bytes([image[12], image[13]]),
        version: text(DESCRIPTION + 16, 32),
        project: text(DESCRIPTION + 48, 32),
        elf_sha256,
        signatures,
    })
}

impl EspImage {
    /// The first nine hex digits of its ELF's SHA-256, as a board's
    /// firmware_version ends with.
    pub fn elf_sha9(&self) -> String {
        self.elf_sha256[..5]
            .iter()
            .map(|octet| format!("{octet:02x}"))
            .collect::<String>()[..9]
            .to_owned()
    }

    /// The chip's name, as ESP-IDF calls it.
    pub fn chip(&self) -> String {
        let name = match self.chip_id {
            0 => "ESP32",
            2 => "ESP32-S2",
            5 => "ESP32-C3",
            9 => "ESP32-S3",
            12 => "ESP32-C2",
            13 => "ESP32-C6",
            16 => "ESP32-H2",
            18 => "ESP32-P4",
            23 => "ESP32-C5",
            _ => return format!("chip {}", self.chip_id),
        };
        name.to_owned()
    }

    /// Whether it is a coprocessor's firmware: its project says so, or is
    /// ESP-Hosted's example, `network_adapter`.
    pub fn is_coprocessor(&self) -> bool {
        let project = self.project.to_ascii_lowercase();
        project.contains("coprocessor") || project == "network_adapter"
    }
}

/// Whether a FIRMWARE_IMAGE memory object, by its object_name, holds a
/// coprocessor's firmware.
pub fn object_is_coprocessor(object_name: &str) -> bool {
    let name = object_name.to_ascii_lowercase();
    name.contains("coprocessor") || name.contains("co-processor")
}

/// The nine hex digits a board's firmware_version ends with, after " - ".
pub fn version_sha9(firmware_version: &str) -> Option<String> {
    let (_, rest) = firmware_version.rsplit_once(" - ")?;
    let digits: String = rest.chars().take_while(char::is_ascii_hexdigit).collect();
    (digits.len() >= 9).then(|| digits[..9].to_lowercase())
}

/// Whether a board whose firmware_version is this runs `image` already.
pub fn runs_already(image: &EspImage, firmware_version: &str) -> bool {
    version_sha9(firmware_version).as_deref() == Some(&image.elf_sha9()[..])
}

/// What a board runs after an upload, from its firmware_version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Running {
    /// The image uploaded.
    New,
    /// The image it ran before: it rolled back.
    Old,
    /// Neither, or its version does not say.
    Other,
}

/// What a board runs after an upload of `image`, from its firmware_version
/// `before` the upload and `after` it.
pub fn running(image: &EspImage, before: &str, after: &str) -> Running {
    match version_sha9(after) {
        Some(sha9) if sha9 == image.elf_sha9() => Running::New,
        Some(sha9) if Some(&sha9) == version_sha9(before).as_ref() => Running::Old,
        _ => Running::Other,
    }
}

/// A doubt about sending an image to a memory object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Doubt {
    /// A coprocessor's firmware for the board's own.
    CoprocessorFileForMain,
    /// The board's own firmware for its coprocessor.
    MainFileForCoprocessor,
    /// No signature: a board that checks signatures refuses it.
    Unsigned,
    /// A coprocessor may keep no old image to roll back to.
    NoRollback,
}

impl Doubt {
    /// Whether it stops an upload unless the person says to go on: the
    /// file is for the other processor.
    pub fn stops(self) -> bool {
        matches!(
            self,
            Doubt::CoprocessorFileForMain | Doubt::MainFileForCoprocessor
        )
    }
}

impl fmt::Display for Doubt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Doubt::CoprocessorFileForMain => {
                "the file is a coprocessor's firmware, and the image is not a coprocessor's"
            }
            Doubt::MainFileForCoprocessor => {
                "the image is a coprocessor's, and the file is not a coprocessor's firmware"
            }
            Doubt::Unsigned => "the file is not signed; a device that checks signatures refuses it",
            Doubt::NoRollback => {
                "a coprocessor may have no old image to roll back to if the new one fails"
            }
        })
    }
}

/// The doubts about sending `image` to the memory object `object_name`.
pub fn check(image: &EspImage, object_name: &str) -> Vec<Doubt> {
    let mut doubts = Vec::new();
    let coprocessor = object_is_coprocessor(object_name);
    match (image.is_coprocessor(), coprocessor) {
        (true, false) => doubts.push(Doubt::CoprocessorFileForMain),
        (false, true) => doubts.push(Doubt::MainFileForCoprocessor),
        _ => {}
    }
    if image.signatures == 0 {
        doubts.push(Doubt::Unsigned);
    }
    if coprocessor {
        doubts.push(Doubt::NoRollback);
    }
    doubts
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An image as idf.py lays one out, `length` octets.
    fn image(length: usize, project: &str, signatures: usize) -> Vec<u8> {
        let mut image = vec![0u8; length];
        image[0] = 0xe9;
        image[12] = 18;
        image[32..36].copy_from_slice(&0xabcd_5432u32.to_le_bytes());
        image[48..53].copy_from_slice(b"1.2.3");
        image[80..80 + project.len()].copy_from_slice(project.as_bytes());
        for (at, octet) in image[176..208].iter_mut().enumerate() {
            *octet = 0xd0 + at as u8;
        }
        let sector = length - 4096;
        for block in 0..signatures {
            image[sector + block * 1216] = 0xe7;
            image[sector + block * 1216 + 1] = 0x02;
        }
        image
    }

    #[test]
    fn an_images_header_and_description_are_read() {
        let described = parse(&image(8192, "avb_endpoint", 2)).unwrap();
        assert_eq!(described.version, "1.2.3");
        assert_eq!(described.project, "avb_endpoint");
        assert_eq!(described.chip(), "ESP32-P4");
        assert_eq!(described.elf_sha9(), "d0d1d2d3d");
        assert_eq!(described.signatures, 2);
        assert!(!described.is_coprocessor());
        assert_eq!(parse(&image(8000, "x", 0)).unwrap().signatures, 0);
        let mut bad = image(8192, "x", 0);
        bad[0] = 0;
        assert_eq!(parse(&bad), Err(ImageError::NoHeader));
        let mut bad = image(8192, "x", 0);
        bad[32] = 0;
        assert_eq!(parse(&bad), Err(ImageError::NoDescription));
    }

    #[test]
    fn a_boards_version_tells_which_image_it_runs() {
        let new = parse(&image(8192, "avb_endpoint", 1)).unwrap();
        let before = "1.0.0 (esp_avb 2.45.0 - dccd7ef2f)";
        assert_eq!(version_sha9(before).as_deref(), Some("dccd7ef2f"));
        assert_eq!(version_sha9("0.3.2"), None);
        assert!(runs_already(&new, "1.0.1 (esp_avb 2.45.0 - d0d1d2d3d)"));
        assert!(!runs_already(&new, before));
        assert_eq!(
            running(&new, before, "1.0.1 (esp_avb 2.45.0 - d0d1d2d3d)"),
            Running::New
        );
        assert_eq!(running(&new, before, before), Running::Old);
        assert_eq!(running(&new, before, "2.0.0"), Running::Other);
    }

    #[test]
    fn a_file_for_the_other_processor_stops_an_upload() {
        let main = parse(&image(8192, "avb_endpoint", 1)).unwrap();
        let hosted = parse(&image(8192, "network_adapter", 0)).unwrap();
        let named = parse(&image(8192, "esp_ap_coprocessor", 1)).unwrap();
        assert!(hosted.is_coprocessor() && named.is_coprocessor());
        assert!(object_is_coprocessor("Coprocessor firmware"));
        assert!(object_is_coprocessor("Wi-Fi co-processor"));
        assert!(check(&main, "Firmware").is_empty());
        assert_eq!(check(&named, "Firmware"), [Doubt::CoprocessorFileForMain]);
        assert_eq!(
            check(&main, "Coprocessor firmware"),
            [Doubt::MainFileForCoprocessor, Doubt::NoRollback]
        );
        assert_eq!(
            check(&hosted, "Coprocessor firmware"),
            [Doubt::Unsigned, Doubt::NoRollback]
        );
        assert!(Doubt::CoprocessorFileForMain.stops() && !Doubt::Unsigned.stops());
    }
}
