//! Entities' logos, their own and their vendors', as triib reads them
//! through Address Access: each decoded once, the first time it shows, and
//! shared by every entity with the same image, as a vendor's are.

use std::cell::RefCell;
use std::collections::HashMap;

use atdecc::descriptor::MemoryObjectType;
use atdecc::model::EntityModel;
use iced::widget::image::Handle;

/// The longest side of a logo decoded, against images no logo needs.
const MAX_SIDE: u32 = 1024;

/// Which of an entity's logos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Logo {
    Entity,
    Vendor,
}

impl Logo {
    fn memory_object_type(self) -> MemoryObjectType {
        match self {
            Self::Entity => MemoryObjectType::PNG_ENTITY,
            Self::Vendor => MemoryObjectType::PNG_MANUFACTURER,
        }
    }
}

/// The logos decoded, by their PNG's FNV-1a hash and length; `None` for
/// one that does not decode.
#[derive(Default)]
pub struct Logos(RefCell<HashMap<(u64, usize), Option<Handle>>>);

impl Logos {
    /// An entity's logo, once read and if it decodes.
    pub fn of(&self, model: &EntityModel, logo: Logo) -> Option<Handle> {
        let png = model.image(logo.memory_object_type())?;
        let hash = png.iter().fold(0xcbf2_9ce4_8422_2325u64, |hash, octet| {
            (hash ^ u64::from(*octet)).wrapping_mul(0x0100_0000_01b3)
        });
        self.0
            .borrow_mut()
            .entry((hash, png.len()))
            .or_insert_with(|| decode(png))
            .clone()
    }
}

/// A PNG as RGBA pixels for iced to draw, at eight bits a channel; `None`
/// for one that does not decode, or larger than any logo needs.
pub fn decode(png: &[u8]) -> Option<Handle> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(png));
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().ok()?;
    let (width, height) = reader.info().size();
    if width > MAX_SIDE || height > MAX_SIDE {
        return None;
    }
    let mut pixels = vec![0; reader.output_buffer_size()?];
    let frame = reader.next_frame(&mut pixels).ok()?;
    let pixels = &pixels[..frame.buffer_size()];
    let rgba: Vec<u8> = match frame.color_type {
        png::ColorType::Rgba => pixels.to_vec(),
        png::ColorType::Rgb => pixels
            .as_chunks::<3>()
            .0
            .iter()
            .flat_map(|&[red, green, blue]| [red, green, blue, 255])
            .collect(),
        png::ColorType::GrayscaleAlpha => pixels
            .as_chunks::<2>()
            .0
            .iter()
            .flat_map(|&[gray, alpha]| [gray, gray, gray, alpha])
            .collect(),
        png::ColorType::Grayscale => pixels
            .iter()
            .flat_map(|&gray| [gray, gray, gray, 255])
            .collect(),
        // Expanded to the others by the transformations.
        png::ColorType::Indexed => return None,
    };
    Some(Handle::from_rgba(frame.width, frame.height, rgba))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triibs_logos_decode() {
        for png in [
            triib_endpointd::runtime::LOGO,
            triib_endpointd::runtime::VENDOR_LOGO,
        ] {
            let Some(Handle::Rgba {
                width,
                height,
                pixels,
                ..
            }) = decode(png)
            else {
                panic!("not decoded");
            };
            assert_eq!((width, height), (128, 128));
            assert_eq!(pixels.len(), 128 * 128 * 4);
        }
        assert!(decode(b"not a PNG").is_none());
    }

    /// A grayscale PNG with alpha, as the vendor logo's source is, comes out
    /// as RGBA.
    #[test]
    fn grayscale_comes_out_as_rgba() {
        let mut png = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut png, 2, 1);
            encoder.set_color(png::ColorType::GrayscaleAlpha);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&[10, 255, 200, 0]).unwrap();
        }
        let Some(Handle::Rgba { pixels, .. }) = decode(&png) else {
            panic!("not decoded");
        };
        assert_eq!(pixels.as_ref(), [10, 10, 10, 255, 200, 200, 200, 0]);
    }
}
