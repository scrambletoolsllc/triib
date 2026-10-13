//! The ways a stream carries samples, AAF and AM824, and a media clock,
//! CRF, behind one layout: building a talker's PDUs and reading a
//! listener's.

use atdecc::stream_format::StreamFormat;

use crate::{aaf, am824, crf};

/// How a stream format packs its samples, or its clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Media {
    Aaf(aaf::Layout),
    Am824(am824::Layout),
    Crf(crf::Layout),
}

impl Media {
    /// The media of `format`, when this crate can send and receive it.
    pub fn of(format: StreamFormat) -> Option<Self> {
        aaf::Layout::of(format)
            .map(Media::Aaf)
            .or_else(|| am824::Layout::of(format).map(Media::Am824))
            .or_else(|| crf::Layout::of(format).map(Media::Crf))
    }

    /// The AVTP subtype of its PDUs.
    pub fn subtype(&self) -> u8 {
        match self {
            Media::Aaf(_) => aaf::SUBTYPE,
            Media::Am824(_) => am824::SUBTYPE,
            Media::Crf(_) => crf::SUBTYPE,
        }
    }

    /// Its sampling rate; a media clock's base frequency.
    pub fn sample_rate(&self) -> u32 {
        match self {
            Media::Aaf(layout) => layout.sample_rate,
            Media::Am824(layout) => layout.sample_rate,
            Media::Crf(layout) => layout.base_frequency,
        }
    }

    /// Its audio channels, none for a media clock.
    pub fn channels(&self) -> u16 {
        match self {
            Media::Aaf(layout) => layout.channels,
            Media::Am824(layout) => layout.channels,
            Media::Crf(_) => 0,
        }
    }

    /// Whether it carries a media clock rather than audio.
    pub fn is_clock(&self) -> bool {
        matches!(self, Media::Crf(_))
    }

    /// The samples each frame carries; none for a media clock.
    pub fn samples_per_frame(&self) -> u16 {
        match self {
            Media::Aaf(layout) => layout.samples_per_frame,
            Media::Am824(layout) => layout.samples_per_frame,
            Media::Crf(_) => 0,
        }
    }

    /// How many samples at `rate` each frame spans: an audio stream's own
    /// at its rate alone; a media clock's events counted at `rate`, where
    /// they come to whole samples.
    pub fn frame_samples_at(&self, rate: u32) -> Option<u64> {
        match self {
            Media::Crf(layout) => {
                let spanned = layout.events_per_frame() * u64::from(rate);
                let base = u64::from(layout.base_frequency);
                (spanned.is_multiple_of(base) && spanned > 0).then(|| spanned / base)
            }
            _ => (self.sample_rate() == rate).then(|| u64::from(self.samples_per_frame().max(1))),
        }
    }

    /// The AVTPDU's octets, as MSRP's MaxFrameSize counts them.
    pub fn pdu_length(&self) -> usize {
        match self {
            Media::Aaf(layout) => layout.pdu_length(),
            Media::Am824(layout) => layout.pdu_length(),
            Media::Crf(layout) => layout.pdu_length(),
        }
    }

    /// How often a frame goes out, in nanoseconds.
    pub fn interval_nanos(&self) -> u64 {
        match self {
            Media::Aaf(layout) => layout.interval_nanos(),
            Media::Am824(layout) => layout.interval_nanos(),
            Media::Crf(layout) => layout.interval_nanos(),
        }
    }
}

/// Builds one stream's PDUs, counting what each carries on.
#[derive(Debug, Clone)]
pub struct Packetizer {
    media: Media,
    stream_id: u64,
    sequence: u8,
    /// AM824's count of data blocks sent.
    dbc: u8,
}

impl Packetizer {
    pub fn new(media: Media, stream_id: u64) -> Self {
        Packetizer {
            media,
            stream_id,
            sequence: 0,
            dbc: 0,
        }
    }

    /// Writes the next PDU into `out`, `media`'s PDU length: `samples`
    /// interleaved, the first presented at `presentation` (gPTP
    /// nanoseconds); for a media clock, its timestamps from
    /// `presentation` on, without samples.
    pub fn write(&mut self, presentation: i64, media_reset: bool, samples: &[f32], out: &mut [u8]) {
        match self.media {
            Media::Aaf(layout) => {
                let header = aaf::Header {
                    sequence: self.sequence,
                    stream_id: self.stream_id,
                    timestamp: Some(presentation as u32),
                    media_reset,
                    nsr: layout.nsr,
                    sample_format: layout.sample_format,
                    bit_depth: layout.bit_depth,
                    channels: layout.channels,
                    data_length: layout.data_length() as u16,
                };
                let mut encoded = [0; aaf::HEADER_LEN];
                header.encode(&mut encoded);
                out[..aaf::HEADER_LEN].copy_from_slice(&encoded);
                aaf::write_samples(samples, &layout, &mut out[aaf::HEADER_LEN..]);
            }
            Media::Am824(layout) => {
                // The timestamp is for the block at an SYT_INTERVAL
                // boundary, and a frame without one has none.
                let timestamp = layout.stamped_block(self.dbc).map(|block| {
                    let offset = i64::from(block) * 1_000_000_000 / i64::from(layout.sample_rate);
                    (presentation + offset) as u32
                });
                let header = am824::Header {
                    sequence: self.sequence,
                    stream_id: self.stream_id,
                    timestamp,
                    media_reset,
                    dbs: layout.channels as u8,
                    sfc: layout.sfc,
                    dbc: self.dbc,
                    data_length: layout.data_length() as u16,
                };
                let start = am824::HEADER_LEN + am824::CIP_LEN;
                let mut encoded = [0; am824::HEADER_LEN + am824::CIP_LEN];
                header.encode(&mut encoded);
                out[..start].copy_from_slice(&encoded);
                am824::write_samples(samples, &mut out[start..]);
                self.dbc = self.dbc.wrapping_add(layout.samples_per_frame as u8);
            }
            Media::Crf(layout) => {
                let mut encoded = [0; crf::HEADER_LEN];
                crf::Header::of(&layout, self.sequence, self.stream_id, media_reset)
                    .encode(&mut encoded);
                out[..crf::HEADER_LEN].copy_from_slice(&encoded);
                crf::write_timestamps(presentation, &layout, &mut out[crf::HEADER_LEN..]);
            }
        }
        self.sequence = self.sequence.wrapping_add(1);
    }
}

/// What a listener takes from one PDU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Received {
    pub sequence: u8,
    pub stream_id: u64,
    /// A presentation time it carries: the low 32 bits of gPTP
    /// nanoseconds.
    pub timestamp: Option<u32>,
    /// The presentation time of its first sample, where it carries one.
    pub first_presented: Option<u32>,
    /// Its media reset bit, which the talker toggles as its media clock
    /// starts again.
    pub media_reset: bool,
    /// It carries what the media describes.
    pub matches: bool,
}

/// Reads a PDU of `media`'s kind, its samples into `out` when it carries
/// what `media` describes; a media clock's has none.
pub fn read(pdu: &[u8], media: &Media, out: &mut Vec<f32>) -> Option<Received> {
    match media {
        Media::Aaf(layout) => {
            let (header, data) = aaf::Header::decode(pdu)?;
            let matches = header.matches(layout);
            if matches {
                aaf::read_samples(data, layout, out);
            }
            Some(Received {
                sequence: header.sequence,
                stream_id: header.stream_id,
                timestamp: header.timestamp,
                first_presented: header.timestamp,
                media_reset: header.media_reset,
                matches,
            })
        }
        Media::Am824(layout) => {
            let (header, data) = am824::Header::decode(pdu)?;
            let matches = header.matches(layout);
            if matches {
                am824::read_samples(data, out);
            }
            // The timestamp is for the block at the SYT_INTERVAL boundary.
            let block = layout.stamped_block(header.dbc).unwrap_or(0);
            let before = u64::from(block) * 1_000_000_000 / u64::from(layout.sample_rate);
            Some(Received {
                sequence: header.sequence,
                stream_id: header.stream_id,
                timestamp: header.timestamp,
                first_presented: header
                    .timestamp
                    .map(|timestamp| timestamp.wrapping_sub(before as u32)),
                media_reset: header.media_reset,
                matches,
            })
        }
        Media::Crf(layout) => {
            let (header, timestamps) = crf::Header::decode(pdu)?;
            let first = crf::first_timestamp(timestamps).map(|time| time as u32);
            Some(Received {
                sequence: header.sequence,
                stream_id: header.stream_id,
                timestamp: first,
                first_presented: first,
                media_reset: header.media_reset,
                matches: header.matches(layout),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AAF_48K_8CH_32: StreamFormat = StreamFormat(0x0205_0220_0200_6000);
    const AM824_48K_8CH: StreamFormat = StreamFormat(0x00a0_0208_4000_0800);

    #[test]
    fn both_kinds_go_out_and_come_back() {
        for format in [AAF_48K_8CH_32, AM824_48K_8CH] {
            let media = Media::of(format).unwrap();
            assert_eq!(media.interval_nanos(), 125_000);
            let mut packetizer = Packetizer::new(media, 0x9c6b_0030_9a2b_0000);
            let samples: Vec<f32> = (0..48).map(|place| place as f32 / 64.0 - 0.3).collect();
            let mut stamps = Vec::new();
            for frame in 0..4u8 {
                let mut pdu = vec![0; media.pdu_length()];
                let presentation = 1_000_000 + i64::from(frame) * 125_000;
                packetizer.write(presentation, false, &samples, &mut pdu);
                let mut back = Vec::new();
                let received = read(&pdu, &media, &mut back).unwrap();
                assert_eq!(received.sequence, frame);
                assert!(received.matches);
                assert_eq!(back.len(), 48);
                stamps.push(received.timestamp);
            }
            match media {
                Media::Aaf(_) => assert_eq!(
                    stamps,
                    [
                        Some(1_000_000),
                        Some(1_125_000),
                        Some(1_250_000),
                        Some(1_375_000)
                    ]
                ),
                // Blocks 0, 8 and 16 are stamped, 2 and 4 blocks into their
                // frames; the fourth frame has no boundary.
                Media::Am824(_) => assert_eq!(
                    stamps,
                    [Some(1_000_000), Some(1_166_666), Some(1_333_333), None]
                ),
                Media::Crf(_) => unreachable!("audio formats only"),
            }
        }
    }

    /// A media clock's frame spans whole samples at the audio's rates:
    /// 96 events at 48 kHz are 384 samples at 192 kHz; an audio stream
    /// spans its own frame at its own rate alone.
    #[test]
    fn a_media_clock_spans_samples_at_any_audio_rate() {
        let clock = Media::of(StreamFormat(0x0410_6001_0000_bb80)).unwrap();
        assert!(clock.is_clock());
        assert_eq!(clock.channels(), 0);
        assert_eq!(clock.frame_samples_at(48_000), Some(96));
        assert_eq!(clock.frame_samples_at(192_000), Some(384));
        assert_eq!(clock.frame_samples_at(44_100), None);
        let audio = Media::of(AAF_48K_8CH_32).unwrap();
        assert_eq!(audio.frame_samples_at(48_000), Some(6));
        assert_eq!(audio.frame_samples_at(96_000), None);
        let mut pdu = vec![0; clock.pdu_length()];
        Packetizer::new(clock, 1).write(3_000_000_000, false, &[], &mut pdu);
        let received = read(&pdu, &clock, &mut Vec::new()).unwrap();
        assert!(received.matches);
        assert_eq!(received.timestamp, Some(3_000_000_000));
    }

    #[test]
    fn a_listener_of_one_kind_ignores_the_other() {
        let aaf = Media::of(AAF_48K_8CH_32).unwrap();
        let am824 = Media::of(AM824_48K_8CH).unwrap();
        let mut pdu = vec![0; aaf.pdu_length()];
        Packetizer::new(aaf, 1).write(0, false, &[0.0; 48], &mut pdu);
        assert!(read(&pdu, &am824, &mut Vec::new()).is_none());
    }
}
