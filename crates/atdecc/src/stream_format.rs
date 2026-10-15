//! Stream formats (IEEE 1722-2016, Annex I; IEEE 1722.1-2021, 7.3.3):
//! the 64-bit description of what a stream carries, decoded for the
//! formats audio devices use, and whether a talker's stream suits a
//! listener.

use core::fmt;

/// A stream format, kept as its 64 bits.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct StreamFormat(pub u64);

/// AVTP subtypes that appear in stream formats (IEEE 1722-2016, Table 6).
mod subtype {
    pub const IEC_61883_IIDC: u8 = 0x00;
    pub const AAF: u8 = 0x02;
    pub const CRF: u8 = 0x04;
}

/// What a stream format describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatKind {
    /// AVTP Audio Format (I.2.4).
    Aaf(Aaf),
    /// IEC 61883-6 audio and music data (I.2.2.3.3).
    Iec61883_6(Iec61883_6),
    /// Clock Reference Format (I.2.6).
    Crf(Crf),
    /// A vendor defined format (v set).
    Vendor,
    /// Another AVTP subtype, not decoded.
    Other { subtype: u8 },
}

/// An AAF stream format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Aaf {
    /// The source or sink can use fewer channels than `channels`.
    pub up_to: bool,
    /// The nominal sample rate code (IEEE 1722-2016, Table 11).
    pub nsr: u8,
    /// The sample format code (IEEE 1722-2016, Table 9).
    pub sample_format: u8,
    pub bit_depth: u8,
    pub channels: u16,
    pub samples_per_frame: u16,
}

/// An IEC 61883-6 stream format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Iec61883_6 {
    /// The packetization: AM824, 32-bit fixed point or 32-bit float.
    pub packing: Packing,
    /// The sampling frequency code of the FDF field.
    pub sfc: u8,
    /// Data block size: quadlets, so channels, per data block.
    pub dbs: u8,
    pub blocking: bool,
    pub non_blocking: bool,
    pub up_to: bool,
    pub synchronous: bool,
    /// For AM824: IEC 60958, multi-bit linear audio, MIDI and SMPTE quadlet
    /// counts.
    pub labels: [u8; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Packing {
    Am824,
    Fixed32,
    Float32,
    Other(u8),
}

/// A CRF stream format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Crf {
    /// The clock type (IEEE 1722-2016, Table 26): 1 for audio samples.
    pub crf_type: u8,
    /// Events between timestamps.
    pub timestamp_interval: u16,
    pub timestamps_per_pdu: u8,
    pub pull: u8,
    pub base_frequency: u32,
}

impl StreamFormat {
    fn octet(self, index: usize) -> u8 {
        self.0.to_be_bytes()[index]
    }

    pub fn kind(self) -> FormatKind {
        let first = self.octet(0);
        if first & 0x80 != 0 {
            return FormatKind::Vendor;
        }
        match first & 0x7f {
            subtype::AAF => {
                let low = self.0 as u32;
                FormatKind::Aaf(Aaf {
                    up_to: self.octet(1) & 0x80 != 0,
                    nsr: self.octet(1) & 0x0f,
                    sample_format: self.octet(2),
                    bit_depth: self.octet(3),
                    channels: (low >> 22) as u16,
                    samples_per_frame: ((low >> 12) & 0x3ff) as u16,
                })
            }
            subtype::IEC_61883_IIDC => {
                let sf = self.octet(1) & 0x80 != 0;
                let fmt = (self.octet(1) >> 1) & 0x3f;
                if !sf || fmt != 0x10 {
                    return FormatKind::Other {
                        subtype: subtype::IEC_61883_IIDC,
                    };
                }
                let fdf = self.octet(2);
                let flags = self.octet(4);
                FormatKind::Iec61883_6(Iec61883_6 {
                    packing: match fdf >> 3 {
                        0b00000 => Packing::Am824,
                        0b00110 => Packing::Fixed32,
                        0b00100 => Packing::Float32,
                        other => Packing::Other(other),
                    },
                    sfc: fdf & 0x07,
                    dbs: self.octet(3),
                    blocking: flags & 0x80 != 0,
                    non_blocking: flags & 0x40 != 0,
                    up_to: flags & 0x20 != 0,
                    synchronous: flags & 0x10 != 0,
                    labels: [
                        self.octet(5),
                        self.octet(6),
                        self.octet(7) >> 4,
                        self.octet(7) & 0x0f,
                    ],
                })
            }
            subtype::CRF => {
                let high = (self.0 >> 32) as u32;
                let low = self.0 as u32;
                FormatKind::Crf(Crf {
                    crf_type: ((high >> 20) & 0x0f) as u8,
                    timestamp_interval: ((high >> 8) & 0x0fff) as u16,
                    timestamps_per_pdu: high as u8,
                    pull: (low >> 29) as u8,
                    base_frequency: low & 0x1fff_ffff,
                })
            }
            other => FormatKind::Other { subtype: other },
        }
    }

    /// The nominal sample or clock rate in hertz, where the format gives
    /// one.
    pub fn sample_rate(self) -> Option<u32> {
        match self.kind() {
            FormatKind::Aaf(aaf) => nsr_hertz(aaf.nsr),
            FormatKind::Iec61883_6(iec) => sfc_hertz(iec.sfc),
            FormatKind::Crf(crf) => (crf.pull == 0).then_some(crf.base_frequency),
            _ => None,
        }
    }

    /// Audio channels, for audio formats.
    pub fn channels(self) -> Option<u16> {
        match self.kind() {
            FormatKind::Aaf(aaf) => Some(aaf.channels),
            FormatKind::Iec61883_6(iec) => Some(u16::from(iec.dbs)),
            _ => None,
        }
    }

    /// Whether a stream in it is SR class B's: AAF whose frames each hold
    /// a class B interval's samples, 4000 frames a second. Others go one
    /// frame each class A interval, as Milan's do, or slower, as a media
    /// clock's.
    pub fn is_class_b(self) -> bool {
        match self.kind() {
            FormatKind::Aaf(aaf) => nsr_hertz(aaf.nsr)
                .is_some_and(|rate| u32::from(aaf.samples_per_frame) * 4000 == rate),
            _ => false,
        }
    }

    /// Whether the stream carries a media clock rather than audio.
    pub fn is_clock(self) -> bool {
        matches!(self.kind(), FormatKind::Crf(_))
    }

    /// The bandwidth the stream takes on an Ethernet link in bits per
    /// second, frames counted with their preamble, VLAN tag, frame check
    /// and gap: AAF and AM824 sent 8000 times a second as class A sends
    /// them, CRF as often as its timestamps fill a frame. `None` for a
    /// format whose frame size is not known.
    pub fn wire_bandwidth(self) -> Option<u64> {
        // Preamble and start of frame, header with a VLAN tag, frame check
        // and interframe gap.
        const OVERHEAD: u64 = 8 + 18 + 4 + 12;
        const AVTP_HEADER: u64 = 24;
        let (payload, frames_per_second) = match self.kind() {
            FormatKind::Aaf(aaf) => {
                let octets = match aaf.sample_format {
                    // 32-bit float, 32-bit integer and AES3 subframes.
                    1 | 2 | 5 => 4,
                    3 => 3,
                    4 => 2,
                    _ => return None,
                };
                let samples = u64::from(aaf.samples_per_frame);
                if samples == 0 {
                    return None;
                }
                let payload = AVTP_HEADER + u64::from(aaf.channels) * samples * octets;
                (payload, u64::from(nsr_hertz(aaf.nsr)?) / samples)
            }
            FormatKind::Iec61883_6(iec) => {
                // Data blocks a frame carries 8000 times a second, each
                // after the CIP header.
                let blocks = u64::from(sfc_hertz(iec.sfc)?).div_ceil(8000);
                let payload = AVTP_HEADER + 8 + blocks * u64::from(iec.dbs) * 4;
                (payload, 8000)
            }
            FormatKind::Crf(crf) => {
                let per_frame =
                    u64::from(crf.timestamp_interval) * u64::from(crf.timestamps_per_pdu);
                if per_frame == 0 {
                    return None;
                }
                let payload = 20 + 8 * u64::from(crf.timestamps_per_pdu);
                (payload, u64::from(crf.base_frequency).div_ceil(per_frame))
            }
            _ => return None,
        };
        Some((payload + OVERHEAD) * 8 * frames_per_second)
    }

    /// Whether two audio formats pack samples alike but for their rate:
    /// the same AAF sample format, depth and channels sent as many frames
    /// a second, so in the same SR class, or the same IEC 61883-6
    /// packing, quadlets and labels. An entity changing its sampling rate
    /// moves its streams to the format that matches this way.
    pub fn same_but_rate(self, other: StreamFormat) -> bool {
        match (self.kind(), other.kind()) {
            (FormatKind::Aaf(one), FormatKind::Aaf(two)) => {
                let frames = |aaf: &Aaf, rate: Option<u32>| {
                    rate.map(|rate| u64::from(aaf.samples_per_frame) * u64::from(rate))
                };
                (one.up_to, one.sample_format, one.bit_depth, one.channels)
                    == (two.up_to, two.sample_format, two.bit_depth, two.channels)
                    && frames(&one, nsr_hertz(two.nsr)) == frames(&two, nsr_hertz(one.nsr))
            }
            (FormatKind::Iec61883_6(one), FormatKind::Iec61883_6(two)) => {
                one.packing == two.packing
                    && one.dbs == two.dbs
                    && one.labels == two.labels
                    && (one.blocking, one.non_blocking, one.up_to, one.synchronous)
                        == (two.blocking, two.non_blocking, two.up_to, two.synchronous)
            }
            _ => false,
        }
    }

    /// An AAF format without its up-to flag and channel count, which may
    /// differ between a talker and a listener that takes fewer channels.
    fn aaf_without_channels(self) -> u64 {
        const UP_TO: u64 = 0x0080_0000_0000_0000;
        const CHANNELS: u64 = 0x3ff << 22;
        self.0 & !UP_TO & !CHANNELS
    }

    /// Whether a stream in `self`, a talker's current format, can be
    /// received by a stream input that supports `listener`: the same
    /// format, or one the listener takes with fewer channels.
    pub fn suits(self, listener: StreamFormat) -> bool {
        if self == listener {
            return true;
        }
        match (self.kind(), listener.kind()) {
            (FormatKind::Aaf(talker), FormatKind::Aaf(sink)) => {
                self.aaf_without_channels() == listener.aaf_without_channels()
                    && (talker.channels == sink.channels
                        || sink.up_to && talker.channels <= sink.channels)
            }
            (FormatKind::Iec61883_6(talker), FormatKind::Iec61883_6(sink)) => {
                talker.packing == sink.packing
                    && talker.sfc == sink.sfc
                    && (talker.dbs == sink.dbs || sink.up_to && talker.dbs <= sink.dbs)
                    && (sink.blocking && talker.blocking
                        || sink.non_blocking && talker.non_blocking)
                    && (talker.packing != Packing::Am824
                        || talker.labels == sink.labels
                        || sink.up_to)
            }
            _ => false,
        }
    }
}

/// How a talker's stream fits a listener's stream input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fit {
    /// The input's current format is the talker's.
    Matches,
    /// The input supports the talker's format but is set to another.
    InputMustChange,
    /// The input does not support the talker's format.
    Incompatible,
}

/// How a talker's current format fits a stream input with `current`
/// format and `supported` formats.
pub fn fit(
    talker: StreamFormat,
    current: StreamFormat,
    supported: impl IntoIterator<Item = StreamFormat>,
) -> Fit {
    if talker.suits(current) {
        Fit::Matches
    } else if supported.into_iter().any(|format| talker.suits(format)) {
        Fit::InputMustChange
    } else {
        Fit::Incompatible
    }
}

/// The rate an AAF nsr code stands for (IEEE 1722-2016, Table 11).
pub fn nsr_hertz(nsr: u8) -> Option<u32> {
    Some(match nsr {
        1 => 8_000,
        2 => 16_000,
        3 => 32_000,
        4 => 44_100,
        5 => 48_000,
        6 => 88_200,
        7 => 96_000,
        8 => 176_400,
        9 => 192_000,
        10 => 24_000,
        _ => return None,
    })
}

/// The rate an IEC 61883-6 SFC code stands for.
pub fn sfc_hertz(sfc: u8) -> Option<u32> {
    Some(match sfc {
        0 => 32_000,
        1 => 44_100,
        2 => 48_000,
        3 => 88_200,
        4 => 96_000,
        5 => 176_400,
        6 => 192_000,
        _ => return None,
    })
}

/// "48 kHz", "44.1 kHz".
struct Rate(u32);

impl fmt::Display for Rate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 < 1000 {
            write!(formatter, "{} Hz", self.0)
        } else if self.0.is_multiple_of(1000) {
            write!(formatter, "{} kHz", self.0 / 1000)
        } else {
            write!(formatter, "{}.{} kHz", self.0 / 1000, self.0 % 1000 / 100)
        }
    }
}

impl fmt::Display for StreamFormat {
    /// A short description, such as "AAF 48 kHz, 8 ch, 32-bit (24 used)".
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rate = self.sample_rate().map(Rate);
        match self.kind() {
            FormatKind::Aaf(aaf) => {
                formatter.write_str("AAF ")?;
                if let Some(rate) = rate {
                    write!(formatter, "{rate}, ")?;
                }
                write!(formatter, "{} ch", aaf.channels)?;
                if aaf.up_to {
                    formatter.write_str(" or fewer")?;
                }
                let size = match aaf.sample_format {
                    1 => Some((32, true)),
                    2 => Some((32, false)),
                    3 => Some((24, false)),
                    4 => Some((16, false)),
                    _ => None,
                };
                match size {
                    Some((bits, true)) => write!(formatter, ", {bits}-bit float")?,
                    Some((bits, false))
                        if aaf.bit_depth != 0 && u32::from(aaf.bit_depth) < bits =>
                    {
                        write!(formatter, ", {bits}-bit ({} used)", aaf.bit_depth)?
                    }
                    Some((bits, false)) => write!(formatter, ", {bits}-bit")?,
                    None => write!(formatter, ", sample format {}", aaf.sample_format)?,
                }
                // Milan's formats send a frame each class A interval, 8000
                // a second; one with clearly fewer, as class B's 4000, says
                // how many samples a frame holds.
                match self.sample_rate() {
                    Some(rate)
                        if aaf.samples_per_frame > 0
                            && rate / u32::from(aaf.samples_per_frame) < 6000 =>
                    {
                        write!(formatter, ", {} samples a frame", aaf.samples_per_frame)
                    }
                    _ => Ok(()),
                }
            }
            FormatKind::Iec61883_6(iec) => {
                let packing = match iec.packing {
                    Packing::Am824 => "AM824",
                    Packing::Fixed32 => "IEC 61883-6 32-bit",
                    Packing::Float32 => "IEC 61883-6 float",
                    Packing::Other(_) => "IEC 61883-6",
                };
                write!(formatter, "{packing} ")?;
                if let Some(rate) = rate {
                    write!(formatter, "{rate}, ")?;
                }
                write!(formatter, "{} ch", iec.dbs)?;
                if iec.up_to {
                    formatter.write_str(" or fewer")?;
                }
                // Non-blocking and asynchronous are what AVB streams use;
                // say so only when a format differs.
                if iec.blocking {
                    formatter.write_str(", blocking")?;
                }
                if iec.synchronous {
                    formatter.write_str(", synchronous")?;
                }
                Ok(())
            }
            FormatKind::Crf(crf) => {
                formatter.write_str("CRF ")?;
                let (kind, event) = match crf.crf_type {
                    1 => ("audio", "sample"),
                    2 => ("video frame", "frame"),
                    3 => ("video line", "line"),
                    4 => ("machine cycle", "cycle"),
                    _ => ("clock", "event"),
                };
                write!(formatter, "{kind} {}", Rate(crf.base_frequency))?;
                // The pull multiplies the base frequency (IEEE 1722-2016,
                // Table 27).
                match crf.pull {
                    0 => {}
                    1 => formatter.write_str(" × 1/1.001")?,
                    2 => formatter.write_str(" × 1.001")?,
                    3 => formatter.write_str(" × 24/25")?,
                    4 => formatter.write_str(" × 25/24")?,
                    5 => formatter.write_str(" × 1/8")?,
                    pull => write!(formatter, ", pull {pull}")?,
                }
                match crf.timestamp_interval {
                    1 => write!(formatter, ", every {event}"),
                    interval => write!(formatter, ", every {interval} {event}s"),
                }
            }
            FormatKind::Vendor => write!(formatter, "vendor format 0x{:016x}", self.0),
            FormatKind::Other { subtype } => {
                write!(
                    formatter,
                    "subtype 0x{subtype:02x} format 0x{:016x}",
                    self.0
                )
            }
        }
    }
}

impl fmt::Debug for StreamFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "StreamFormat(0x{:016x})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_compare_but_for_their_rate() {
        // AAF, 8 channels of 32-bit, at 48 and 96 kHz: 6 and 12 samples.
        let aaf_48 = StreamFormat(0x0205_0220_0200_6000);
        let aaf_96 = StreamFormat(0x0207_0220_0200_c000);
        let aaf_24_in_32 = StreamFormat(0x0205_0218_0200_6000);
        assert!(aaf_48.same_but_rate(aaf_96));
        assert!(!aaf_48.same_but_rate(aaf_24_in_32));
        // Nor do half the frames, SR class B's 12 samples at 48 kHz, pass
        // for class A's 12 at 96 kHz, but class B's 24 there do.
        let class_b_48 = StreamFormat(0x0205_0220_0200_c000);
        let class_b_96 = StreamFormat(0x0207_0220_0201_8000);
        assert!(!class_b_48.same_but_rate(aaf_96));
        assert!(class_b_48.same_but_rate(class_b_96));
        assert!(class_b_48.is_class_b() && class_b_96.is_class_b());
        assert!(!aaf_48.is_class_b() && !aaf_96.is_class_b());
        // AM824, 8 channels, at 48 and 96 kHz.
        let am824_48 = StreamFormat(0x00a0_0208_4000_0800);
        let am824_96 = StreamFormat(0x00a0_0408_4000_0800);
        assert!(am824_48.same_but_rate(am824_96));
        assert!(!am824_48.same_but_rate(aaf_48));
    }

    #[test]
    fn bandwidth_follows_the_frames_on_the_wire() {
        // AAF 48 kHz, 8 channels of 32 bits, 6 samples a frame: 216 octets
        // of payload 8000 times a second.
        assert_eq!(
            StreamFormat(0x0205_0220_0200_6000).wire_bandwidth(),
            Some((216 + 42) * 8 * 8000)
        );
        // AM824 48 kHz, 8 channels: 6 data blocks of 8 quadlets.
        assert_eq!(
            StreamFormat(0x00a0_0208_4000_0800).wire_bandwidth(),
            Some((24 + 8 + 192 + 42) * 8 * 8000)
        );
        // CRF 48 kHz, one timestamp each 96 samples: 500 frames a second.
        assert_eq!(
            StreamFormat(0x0410_6001_0000_bb80).wire_bandwidth(),
            Some((28 + 42) * 8 * 500)
        );
        assert_eq!(StreamFormat(0x8000_0000_0000_0000).wire_bandwidth(), None);
    }

    #[test]
    fn decodes_aaf_from_a_milan_endpoint() {
        let format = StreamFormat(0x0205_0218_0200_6000);
        let FormatKind::Aaf(aaf) = format.kind() else {
            panic!("{format:?}");
        };
        assert_eq!((aaf.nsr, aaf.sample_format, aaf.bit_depth), (5, 2, 24));
        assert_eq!((aaf.channels, aaf.samples_per_frame), (8, 6));
        assert_eq!(format.sample_rate(), Some(48_000));
        assert_eq!(format.to_string(), "AAF 48 kHz, 8 ch, 32-bit (24 used)");
        // A frame each class B interval, twice the samples, says so.
        assert_eq!(
            StreamFormat(0x0205_0220_0200_c000).to_string(),
            "AAF 48 kHz, 8 ch, 32-bit, 12 samples a frame"
        );
        assert_eq!(
            StreamFormat(0x0209_0218_0201_8000).to_string(),
            "AAF 192 kHz, 8 ch, 32-bit (24 used)"
        );
    }

    #[test]
    fn decodes_am824_from_macos() {
        let format = StreamFormat(0x00a0_0608_4000_0800);
        let FormatKind::Iec61883_6(iec) = format.kind() else {
            panic!("{format:?}");
        };
        assert_eq!(iec.packing, Packing::Am824);
        assert_eq!((iec.sfc, iec.dbs), (6, 8));
        assert!(iec.non_blocking && !iec.blocking);
        assert_eq!(iec.labels, [0, 8, 0, 0]);
        assert_eq!(format.to_string(), "AM824 192 kHz, 8 ch");
        // The same with the synchronous flag set reads apart.
        assert_eq!(
            StreamFormat(0x00a0_0608_5000_0800).to_string(),
            "AM824 192 kHz, 8 ch, synchronous"
        );
    }

    #[test]
    fn decodes_crf() {
        let format = StreamFormat(0x0410_6001_0000_bb80);
        let FormatKind::Crf(crf) = format.kind() else {
            panic!("{format:?}");
        };
        assert_eq!(
            (crf.crf_type, crf.timestamp_interval, crf.timestamps_per_pdu),
            (1, 96, 1)
        );
        assert_eq!(crf.base_frequency, 48_000);
        assert!(format.is_clock());
        assert_eq!(format.to_string(), "CRF audio 48 kHz, every 96 samples");
        // Video frame clocks run below a kilohertz, some pulled.
        assert_eq!(
            StreamFormat(0x0420_0101_0000_0018).to_string(),
            "CRF video frame 24 Hz, every frame"
        );
        assert_eq!(
            StreamFormat(0x0420_0101_2000_0018).to_string(),
            "CRF video frame 24 Hz × 1/1.001, every frame"
        );
    }

    #[test]
    fn fits_by_format_and_channels() {
        let eight = StreamFormat(0x0205_0220_0200_6000);
        let two = StreamFormat(0x0205_0220_0080_6000);
        let eight_up_to = StreamFormat(eight.0 | 0x0080_0000_0000_0000);
        let other_rate = StreamFormat(0x0207_0220_0200_c000);
        let clock = StreamFormat(0x0410_6001_0000_bb80);
        assert_eq!(fit(eight, eight, []), Fit::Matches);
        assert_eq!(fit(two, eight_up_to, []), Fit::Matches);
        assert_eq!(fit(two, eight, [two]), Fit::InputMustChange);
        assert_eq!(fit(eight, two, []), Fit::Incompatible);
        assert_eq!(fit(other_rate, eight, [eight, two]), Fit::Incompatible);
        assert_eq!(fit(clock, eight, [eight]), Fit::Incompatible);
    }
}
