//! Entities' logos, their own and their vendors', as triib reads them
//! through Address Access: each decoded once for each size it shows at, the
//! first time it does, and shared by every entity with the same image, as a
//! vendor's are.

use std::cell::{Cell, RefCell};
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

/// The logos decoded, by their PNG's FNV-1a hash and length and the side
/// in pixels they fit; `None` for one that does not decode.
pub struct Logos {
    decoded: RefCell<HashMap<(u64, usize, u32), Option<Handle>>>,
    /// The window's pixels to each of its points.
    scale: Cell<f32>,
}

impl Default for Logos {
    fn default() -> Self {
        Self {
            decoded: RefCell::default(),
            scale: Cell::new(1.0),
        }
    }
}

impl Logos {
    /// An entity's logo, once read and if it decodes, to show in a square
    /// `size` points on a side.
    pub fn of(&self, model: &EntityModel, logo: Logo, size: f32) -> Option<Handle> {
        let png = model.image(logo.memory_object_type())?;
        let hash = png.iter().fold(0xcbf2_9ce4_8422_2325u64, |hash, octet| {
            (hash ^ u64::from(*octet)).wrapping_mul(0x0100_0000_01b3)
        });
        let side = (size * self.scale.get()).round().max(1.0) as u32;
        self.decoded
            .borrow_mut()
            .entry((hash, png.len(), side))
            .or_insert_with(|| {
                let pixels = fit(&decode(png)?, side);
                Some(Handle::from_rgba(pixels.width, pixels.height, pixels.rgba))
            })
            .clone()
    }

    /// The window's pixels to each of its points, as it shows on another
    /// screen; the logos are decoded again for it as they show.
    pub fn rescaled(&self, scale: f32) {
        if scale > 0.0 && scale != self.scale.get() {
            self.scale.set(scale);
            self.decoded.borrow_mut().clear();
        }
    }
}

/// An image's pixels, RGBA at eight bits a channel, unpremultiplied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pixels {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// A PNG's pixels; `None` for one that does not decode, or larger than any
/// logo needs.
pub fn decode(png: &[u8]) -> Option<Pixels> {
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
    Some(Pixels {
        width: frame.width,
        height: frame.height,
        rgba,
    })
}

/// Pixels shrunk to fit a square `side` pixels on a side, each the average
/// of those it covers, weighted by their alpha; iced samples only the four
/// nearest, and a logo shrunk more than twice over that way comes out
/// jagged. One that fits already is left as it is.
pub fn fit(pixels: &Pixels, side: u32) -> Pixels {
    let longest = pixels.width.max(pixels.height);
    if longest <= side {
        return pixels.clone();
    }
    let shrunk = |length: u32| {
        ((f64::from(length) * f64::from(side) / f64::from(longest)).round() as u32).max(1)
    };
    let (width, height) = (shrunk(pixels.width), shrunk(pixels.height));
    let premultiplied: Vec<[f64; 4]> = pixels
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|&[red, green, blue, alpha]| {
            let alpha = f64::from(alpha) / 255.0;
            [
                f64::from(red) * alpha,
                f64::from(green) * alpha,
                f64::from(blue) * alpha,
                alpha,
            ]
        })
        .collect();
    let (from_width, to_width) = (pixels.width as usize, width as usize);
    let across = spans(pixels.width, width);
    let mut rows = vec![[0.0; 4]; to_width * pixels.height as usize];
    for (row, out) in rows.chunks_exact_mut(to_width).enumerate() {
        let source = &premultiplied[row * from_width..][..from_width];
        for (pixel, span) in out.iter_mut().zip(&across) {
            for &(column, weight) in span {
                for (channel, value) in pixel.iter_mut().zip(source[column]) {
                    *channel += value * weight;
                }
            }
        }
    }
    let mut shrunk = vec![[0.0; 4]; to_width * height as usize];
    for (out, span) in shrunk
        .chunks_exact_mut(to_width)
        .zip(spans(pixels.height, height))
    {
        for &(row, weight) in &span {
            for (pixel, source) in out.iter_mut().zip(&rows[row * to_width..][..to_width]) {
                for (channel, value) in pixel.iter_mut().zip(source) {
                    *channel += value * weight;
                }
            }
        }
    }
    let rgba = shrunk
        .into_iter()
        .flat_map(|[red, green, blue, alpha]| {
            if alpha <= 0.0 {
                return [0; 4];
            }
            let octet = |value: f64| value.round().clamp(0.0, 255.0) as u8;
            [
                octet(red / alpha),
                octet(green / alpha),
                octet(blue / alpha),
                octet(alpha * 255.0),
            ]
        })
        .collect();
    Pixels {
        width,
        height,
        rgba,
    }
}

/// For each of `to` pixels along a line `from` pixels long, the pixels it
/// covers and how much of it each makes up.
fn spans(from: u32, to: u32) -> Vec<Vec<(usize, f64)>> {
    let ratio = f64::from(from) / f64::from(to);
    (0..to)
        .map(|out| {
            let start = f64::from(out) * ratio;
            let end = start + ratio;
            (start.floor() as usize..(end.ceil() as usize).min(from as usize))
                .map(|pixel| {
                    let covered = end.min(pixel as f64 + 1.0) - start.max(pixel as f64);
                    (pixel, covered / ratio)
                })
                .filter(|&(_, weight)| weight > 0.0)
                .collect()
        })
        .collect()
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
            let pixels = decode(png).expect("decoded");
            assert_eq!((pixels.width, pixels.height), (128, 128));
            assert_eq!(pixels.rgba.len(), 128 * 128 * 4);
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
        let pixels = decode(&png).expect("decoded");
        assert_eq!(pixels.rgba, [10, 10, 10, 255, 200, 200, 200, 0]);
    }

    /// A logo shrunk to fit averages the pixels each of its own covers, so
    /// a fine pattern turns an even gray rather than breaking up.
    #[test]
    fn a_logo_shrunk_averages_what_each_pixel_covers() {
        let checkers = Pixels {
            width: 8,
            height: 4,
            rgba: (0..32)
                .flat_map(|index| {
                    let gray = if (index % 8 + index / 8) % 2 == 0 {
                        0
                    } else {
                        255
                    };
                    [gray, gray, gray, 255]
                })
                .collect(),
        };
        let fitted = fit(&checkers, 3);
        assert_eq!((fitted.width, fitted.height), (3, 2));
        for pixel in fitted.rgba.as_chunks::<4>().0 {
            assert!((126..=129).contains(&pixel[0]), "{pixel:?}");
            assert_eq!(pixel[3], 255);
        }
        assert_eq!(fit(&checkers, 8), checkers);
    }

    /// A clear pixel's color counts for nothing in the average, so a logo
    /// on a clear background keeps its edges' color.
    #[test]
    fn clear_pixels_leave_the_color_alone() {
        let edge = Pixels {
            width: 2,
            height: 1,
            rgba: vec![255, 0, 0, 255, 0, 0, 0, 0],
        };
        assert_eq!(fit(&edge, 1).rgba, [255, 0, 0, 128]);
    }

    /// Triib's own logos fit the list's and the inspector's squares at the
    /// window's scale.
    #[test]
    fn logos_fit_the_size_they_show_at() {
        let pixels = decode(triib_endpointd::runtime::LOGO).expect("decoded");
        let fitted = fit(&pixels, 24);
        assert_eq!((fitted.width, fitted.height), (24, 24));
        assert_eq!(fitted.rgba.len(), 24 * 24 * 4);
        assert_eq!(fit(&pixels, 256), pixels);
    }
}
