//! CRF PDUs (IEEE 1722-2016, 10): a media clock as the gPTP times of
//! every so many of its events, 64 bits each, after a header.

use atdecc::stream_format::{FormatKind, StreamFormat};

/// CRF's AVTP subtype.
pub const SUBTYPE: u8 = 0x04;
/// Octets of the CRF header.
pub const HEADER_LEN: usize = 20;
/// The CRF type of an audio sample clock (Table 26).
pub const AUDIO_SAMPLE: u8 = 1;

/// What a CRF stream carries, from its stream format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    /// Clock events from one timestamp to the next.
    pub timestamp_interval: u16,
    pub timestamps_per_pdu: u8,
    pub base_frequency: u32,
}

impl Layout {
    /// The layout of a CRF stream format, when it is one this crate can
    /// send and receive: an audio sample clock at its nominal rate.
    pub fn of(format: StreamFormat) -> Option<Self> {
        let FormatKind::Crf(crf) = format.kind() else {
            return None;
        };
        (crf.crf_type == AUDIO_SAMPLE
            && crf.pull == 0
            && crf.timestamp_interval > 0
            && crf.timestamps_per_pdu > 0
            && crf.base_frequency > 0)
            .then_some(Layout {
                timestamp_interval: crf.timestamp_interval,
                timestamps_per_pdu: crf.timestamps_per_pdu,
                base_frequency: crf.base_frequency,
            })
    }

    /// Octets of timestamps in each frame.
    pub fn data_length(&self) -> usize {
        8 * usize::from(self.timestamps_per_pdu)
    }

    /// The AVTPDU's octets: header and timestamps, as MSRP's MaxFrameSize
    /// counts them.
    pub fn pdu_length(&self) -> usize {
        HEADER_LEN + self.data_length()
    }

    /// Clock events each frame covers.
    pub fn events_per_frame(&self) -> u64 {
        u64::from(self.timestamp_interval) * u64::from(self.timestamps_per_pdu)
    }

    /// How often a frame goes out, in nanoseconds.
    pub fn interval_nanos(&self) -> u64 {
        1_000_000_000 * self.events_per_frame() / u64::from(self.base_frequency)
    }

    /// The nanoseconds from a frame's first timestamp to its `index`th.
    fn after_first(&self, index: u8) -> i64 {
        let events = u128::from(self.timestamp_interval) * u128::from(index);
        (events * 1_000_000_000 / u128::from(self.base_frequency)) as i64
    }
}

/// A CRF header's fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub sequence: u8,
    pub stream_id: u64,
    pub media_reset: bool,
    /// The timestamps may be off, as while the clock settles (tu).
    pub uncertain: bool,
    pub crf_type: u8,
    pub pull: u8,
    pub base_frequency: u32,
    pub timestamp_interval: u16,
    pub data_length: u16,
}

impl Header {
    /// The header of a frame of `layout`.
    pub fn of(layout: &Layout, sequence: u8, stream_id: u64, media_reset: bool) -> Self {
        Header {
            sequence,
            stream_id,
            media_reset,
            uncertain: false,
            crf_type: AUDIO_SAMPLE,
            pull: 0,
            base_frequency: layout.base_frequency,
            timestamp_interval: layout.timestamp_interval,
            data_length: layout.data_length() as u16,
        }
    }

    pub fn encode(&self, out: &mut [u8; HEADER_LEN]) {
        out[0] = SUBTYPE;
        // sv, version 0, mr, fs 0, tu.
        out[1] = 0x80 | if self.media_reset { 0x08 } else { 0 } | u8::from(self.uncertain);
        out[2] = self.sequence;
        out[3] = self.crf_type;
        out[4..12].copy_from_slice(&self.stream_id.to_be_bytes());
        let frequency = (u32::from(self.pull) << 29) | (self.base_frequency & 0x1fff_ffff);
        out[12..16].copy_from_slice(&frequency.to_be_bytes());
        out[16..18].copy_from_slice(&self.data_length.to_be_bytes());
        out[18..20].copy_from_slice(&self.timestamp_interval.to_be_bytes());
    }

    /// Decodes a CRF PDU, returning its header and timestamps.
    pub fn decode(pdu: &[u8]) -> Option<(Header, &[u8])> {
        let header: &[u8; HEADER_LEN] = pdu.get(..HEADER_LEN)?.try_into().ok()?;
        if header[0] != SUBTYPE || header[1] & 0x80 == 0 {
            return None;
        }
        let data_length = u16::from_be_bytes([header[16], header[17]]);
        let timestamps = pdu.get(HEADER_LEN..HEADER_LEN + usize::from(data_length))?;
        let mut stream_id = [0; 8];
        stream_id.copy_from_slice(&header[4..12]);
        let frequency = u32::from_be_bytes([header[12], header[13], header[14], header[15]]);
        Some((
            Header {
                sequence: header[2],
                stream_id: u64::from_be_bytes(stream_id),
                media_reset: header[1] & 0x08 != 0,
                uncertain: header[1] & 0x01 != 0,
                crf_type: header[3],
                pull: (frequency >> 29) as u8,
                base_frequency: frequency & 0x1fff_ffff,
                timestamp_interval: u16::from_be_bytes([header[18], header[19]]),
                data_length,
            },
            timestamps,
        ))
    }

    /// Whether the frame carries the clock `layout` describes.
    pub fn matches(&self, layout: &Layout) -> bool {
        self.crf_type == AUDIO_SAMPLE
            && self.pull == 0
            && self.base_frequency == layout.base_frequency
            && self.timestamp_interval == layout.timestamp_interval
            && usize::from(self.data_length) == layout.data_length()
    }
}

/// Writes a frame's timestamps, the first `first` (gPTP nanoseconds) and
/// each next one a timestamp interval on.
pub fn write_timestamps(first: i64, layout: &Layout, out: &mut [u8]) {
    for index in 0..layout.timestamps_per_pdu {
        let at = 8 * usize::from(index);
        let time = (first + layout.after_first(index)) as u64;
        out[at..at + 8].copy_from_slice(&time.to_be_bytes());
    }
}

/// A frame's first timestamp, in gPTP nanoseconds.
pub fn first_timestamp(timestamps: &[u8]) -> Option<u64> {
    Some(u64::from_be_bytes(timestamps.get(..8)?.try_into().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Milan's media clock: 48 kHz, a timestamp every 96 samples, one a
    /// frame, 500 frames a second.
    const MILAN: StreamFormat = StreamFormat(0x0410_6001_0000_bb80);

    #[test]
    fn milans_media_clock_goes_out_and_comes_back() {
        let layout = Layout::of(MILAN).unwrap();
        assert_eq!(layout.pdu_length(), 28);
        assert_eq!(layout.interval_nanos(), 2_000_000);
        let mut pdu = vec![0; layout.pdu_length()];
        let mut header = [0; HEADER_LEN];
        Header::of(&layout, 7, 0x9c6b_0030_9a2b_0001, false).encode(&mut header);
        pdu[..HEADER_LEN].copy_from_slice(&header);
        write_timestamps(5_000_000_123, &layout, &mut pdu[HEADER_LEN..]);
        let (back, timestamps) = Header::decode(&pdu).unwrap();
        assert_eq!(back.sequence, 7);
        assert_eq!(back.stream_id, 0x9c6b_0030_9a2b_0001);
        assert!(back.matches(&layout));
        assert_eq!(first_timestamp(timestamps), Some(5_000_000_123));
        // Its octets as 1722 lays them out.
        assert_eq!(
            &pdu[..4],
            [SUBTYPE, 0x80, 7, AUDIO_SAMPLE],
            "subtype, sv, sequence, type"
        );
        assert_eq!(&pdu[12..20], [0, 0, 0xbb, 0x80, 0, 8, 0, 96]);
    }

    /// Several timestamps a frame are a timestamp interval apart, to the
    /// nanosecond.
    #[test]
    fn timestamps_a_frame_are_an_interval_apart() {
        let layout = Layout {
            timestamp_interval: 160,
            timestamps_per_pdu: 6,
            base_frequency: 48_000,
        };
        let mut data = vec![0; layout.data_length()];
        write_timestamps(1_000_000, &layout, &mut data);
        let times: Vec<u64> = data
            .chunks(8)
            .map(|chunk| u64::from_be_bytes(chunk.try_into().unwrap()))
            .collect();
        assert_eq!(
            times,
            [
                1_000_000, 4_333_333, 7_666_666, 11_000_000, 14_333_333, 17_666_666
            ]
        );
        assert_eq!(layout.interval_nanos(), 20_000_000);
    }

    /// Video frame clocks, and pulled ones, are not sent.
    #[test]
    fn only_audio_sample_clocks_are_taken() {
        assert!(Layout::of(StreamFormat(0x0420_0101_0000_0018)).is_none());
        assert!(Layout::of(StreamFormat(0x0410_6001_2000_bb80)).is_none());
        assert!(Layout::of(StreamFormat(0x0205_0220_0200_6000)).is_none());
    }
}
