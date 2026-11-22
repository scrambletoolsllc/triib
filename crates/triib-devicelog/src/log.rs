//! A device's log, read from its LOG_OBJECT memory object while followed
//! (see [`atdecc::log`]): one read out per device at a time, every 250 ms
//! while the log grows, the header alone every 250 ms after it stops, and
//! back to back while the reader is behind.
//!
//! A [`Reader`] turns what the reads bring into [`Entry`]s: whole lines,
//! and between them the boot each begins, a restart, octets the device's
//! ring let go before they were read, and lines it dropped whole. It
//! counts what reading costs as it goes ([`Stats`]): Address Access
//! frames and octets each way, round trips, timeouts and refusals, and
//! each line's latency from when the device logged it to when it was
//! read. A line's latency is the device's milliseconds since boot at the
//! read, from the log header, less the line's own `(ms)`, plus half the
//! round trip.

use std::time::Duration;

use atdecc::Instant;
use atdecc::aecp::AaStatus;
use atdecc::controller::MemoryRead;
use atdecc::descriptor::MemoryObjectType;
use atdecc::model::EntityModel;

pub use atdecc::log::{LogHeader, LogPiece, LogTail, reset_reason_name};

/// Where an entity's log object starts, if it keeps one.
pub fn log_object(model: &EntityModel) -> Option<u64> {
    model
        .memory_objects()
        .find(|object| object.memory_object_type == MemoryObjectType::LOG_OBJECT)
        .map(|object| object.start_address)
}

/// What reading a log brings, in order: its lines, and what happened
/// between them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry {
    /// A whole line, without its newline.
    Line(String),
    /// The header of the boot read first, and of each boot after a
    /// restart: its ID, how long it had run, why it began and how many
    /// boots began unexpectedly, where the device says.
    Boot(LogHeader),
    /// The device booted again: its log starts over.
    Restarted,
    /// Octets the device's ring let go before they could be read, written
    /// faster than read.
    Lost(u64),
    /// Lines the device dropped whole, never written.
    Dropped(u32),
}

/// What reading a log has cost, and what it found.
#[derive(Debug, Clone, Default)]
pub struct Stats {
    /// Address Access READ commands sent, each once however often it was
    /// sent again.
    pub reads: u64,
    pub frames_out: u64,
    pub octets_out: u64,
    pub frames_in: u64,
    pub octets_in: u64,
    pub timeouts: u64,
    pub refused: u64,
    pub round_trips: Vec<Duration>,
    pub log_octets: u64,
    pub lost: u64,
    pub dropped_lines: u64,
    pub restarts: u64,
    pub lines: u64,
    pub latencies_ms: Vec<u32>,
}

impl Stats {
    /// Frames sent again after going unanswered.
    pub fn retries(&self) -> u64 {
        self.frames_out.saturating_sub(self.reads)
    }

    fn round_trip_ms(&self) -> (f64, f64) {
        let ms = |duration: Duration| duration.as_secs_f64() * 1000.0;
        let average = if self.round_trips.is_empty() {
            0.0
        } else {
            ms(self.round_trips.iter().sum::<Duration>()) / self.round_trips.len() as f64
        };
        let most = ms(self.round_trips.iter().max().copied().unwrap_or_default());
        (average, most)
    }

    /// The median, 95th percentile and longest latency, in milliseconds.
    fn latency_ms(&self) -> (Option<u32>, Option<u32>, Option<u32>) {
        let mut sorted = self.latencies_ms.clone();
        sorted.sort_unstable();
        (
            percentile(&sorted, 0.5),
            percentile(&sorted, 0.95),
            sorted.last().copied(),
        )
    }

    /// The stats on one line, for `label`'s log read for `elapsed`.
    pub fn line(&self, label: &str, elapsed: Duration) -> String {
        let seconds = elapsed.as_secs_f64().max(0.001);
        let (rtt_avg, rtt_max) = self.round_trip_ms();
        let latency = match self.latency_ms() {
            (Some(median), Some(p95), Some(max)) => {
                format!("latency {median}/{p95}/{max} ms (median/p95/max)")
            }
            _ => "latency n/a".into(),
        };
        format!(
            "{label}: {seconds:.0} s, {:.2} reads/s, out {:.2} frames/s {:.0} B/s, in {:.2} \
             frames/s {:.0} B/s, rtt {rtt_avg:.2}/{rtt_max:.2} ms (avg/max), {latency}, {} \
             retries, {} timeouts, {} refused, {} lines, {} B read, {} B lost, {} lines \
             dropped, {} restarts",
            self.reads as f64 / seconds,
            self.frames_out as f64 / seconds,
            self.octets_out as f64 / seconds,
            self.frames_in as f64 / seconds,
            self.octets_in as f64 / seconds,
            self.retries(),
            self.timeouts,
            self.refused,
            self.lines,
            self.log_octets,
            self.lost,
            self.dropped_lines,
            self.restarts,
        )
    }

    /// The stats as a JSON object, for the device `entity` named `name`,
    /// its log read for `elapsed`, in `boot`.
    pub fn json(
        &self,
        entity: &str,
        name: &str,
        boot: Option<&LogHeader>,
        elapsed: Duration,
    ) -> String {
        let (rtt_avg, rtt_max) = self.round_trip_ms();
        let (median, p95, max) = self.latency_ms();
        let number = |value: Option<u32>| value.map_or("null".into(), |value| value.to_string());
        format!(
            "{{\"entity\":\"{entity}\",\"name\":\"{}\",\"seconds\":{:.1},\"reads\":{},\
             \"frames_out\":{},\"octets_out\":{},\"frames_in\":{},\"octets_in\":{},\
             \"retries\":{},\"timeouts\":{},\"refused\":{},\"rtt_ms_avg\":{rtt_avg:.3},\
             \"rtt_ms_max\":{rtt_max:.3},\"lines\":{},\"log_octets\":{},\"lost_octets\":{},\
             \"dropped_lines\":{},\"restarts\":{},\"latency_ms_median\":{},\
             \"latency_ms_p95\":{},\"latency_ms_max\":{},\"boot\":{},\
             \"reset_reason\":{},\"resets\":{}}}",
            json_text(name),
            elapsed.as_secs_f64(),
            self.reads,
            self.frames_out,
            self.octets_out,
            self.frames_in,
            self.octets_in,
            self.retries(),
            self.timeouts,
            self.refused,
            self.lines,
            self.log_octets,
            self.lost,
            self.dropped_lines,
            self.restarts,
            number(median),
            number(p95),
            number(max),
            boot.map_or("null".into(), |boot| format!("\"{:08x}\"", boot.boot)),
            number(boot.and_then(|boot| boot.reset_reason).map(u32::from)),
            number(boot.and_then(|boot| boot.resets).map(u32::from)),
        )
    }
}

/// One device's log as it is followed: what is still to read, the line
/// being put together, the boot read, and what reading has cost.
#[derive(Debug, Clone)]
pub struct Reader {
    tail: LogTail,
    line: Vec<u8>,
    /// Octets up to the next newline belong to a line begun before them.
    skip_partial: bool,
    /// The device's milliseconds since boot when its log was first read
    /// this boot: lines logged before then were waiting, not late.
    reading_since_ms: Option<u32>,
    /// The last round trip, for the latency of the lines it brought.
    last_round_trip: Duration,
    boot: Option<LogHeader>,
    stats: Stats,
}

impl Reader {
    /// Follows the log whose object starts at `base`, reading first at
    /// `now`.
    pub fn new(base: u64, now: Instant) -> Self {
        Reader {
            tail: LogTail::new(base, now),
            line: Vec::new(),
            skip_partial: false,
            reading_since_ms: None,
            last_round_trip: Duration::ZERO,
            boot: None,
            stats: Stats::default(),
        }
    }

    /// Reads octets every `every`, never the header alone: the most
    /// reading costs, for measuring it.
    pub fn poll_every(&mut self, every: Duration) {
        self.tail.set_intervals(every, every);
        self.tail.set_always_read(true);
    }

    /// When the next read is due, none while one is out or the device is
    /// away.
    pub fn poll_timeout(&self) -> Option<Instant> {
        self.tail.poll_timeout()
    }

    /// The next read when it is due, as READ TLVs of addresses and
    /// lengths, to go in one Address Access command.
    pub fn poll_read(&mut self, now: Instant) -> Option<Vec<(u64, u16)>> {
        let reads = self.tail.poll_read(now)?;
        self.stats.reads += 1;
        Some(reads)
    }

    /// Takes the answer to the read out, appending what it says to
    /// `entries`.
    pub fn handle_read(&mut self, now: Instant, read: &MemoryRead, entries: &mut Vec<Entry>) {
        if read.status != AaStatus::SUCCESS {
            self.stats.refused += 1;
            self.tail.handle_failure(now);
            return;
        }
        let tlvs: Vec<(u64, &[u8])> = read
            .tlvs
            .iter()
            .map(|(address, octets)| (*address, &octets[..]))
            .collect();
        let mut pieces = Vec::new();
        self.tail.handle_read(now, &tlvs, &mut pieces);
        for piece in pieces {
            self.take(piece, entries);
        }
    }

    /// The read out went unanswered, or the command failed otherwise: the
    /// next waits a little.
    pub fn handle_failure(&mut self, now: Instant, timed_out: bool) {
        if timed_out {
            self.stats.timeouts += 1;
        }
        self.tail.handle_failure(now);
    }

    /// A frame of `length` octets after the Ethernet header went to the
    /// device with a read.
    pub fn sent(&mut self, length: usize) {
        self.stats.frames_out += 1;
        self.stats.octets_out += wire_octets(length);
    }

    /// A frame of `length` octets came back with an answer, `round_trip`
    /// after its command went, where that is known.
    pub fn received(&mut self, length: usize, round_trip: Option<Duration>) {
        self.stats.frames_in += 1;
        self.stats.octets_in += wire_octets(length);
        if let Some(round_trip) = round_trip {
            self.last_round_trip = round_trip;
            self.stats.round_trips.push(round_trip);
        }
    }

    /// The device went away: nothing is read until it is back.
    pub fn pause(&mut self) {
        self.tail.pause();
    }

    /// The device is back: reading goes on where it left off.
    pub fn resume(&mut self, now: Instant) {
        self.tail.resume(now);
    }

    /// Whether everything the device had logged at the last read has been
    /// read.
    pub fn caught_up(&self) -> bool {
        self.tail.caught_up()
    }

    /// The boot read, as its first read found it.
    pub fn boot(&self) -> Option<&LogHeader> {
        self.boot.as_ref()
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// A line left without its newline, once reading ends.
    pub fn finish(&mut self) -> Option<String> {
        if self.line.is_empty() {
            return None;
        }
        let text = String::from_utf8_lossy(&self.line).into_owned();
        self.line.clear();
        Some(text)
    }

    fn take(&mut self, piece: LogPiece, entries: &mut Vec<Entry>) {
        match piece {
            LogPiece::Text {
                octets,
                board_ms,
                after_gap,
            } => {
                self.stats.log_octets += octets.len() as u64;
                let since = *self.reading_since_ms.get_or_insert(board_ms);
                if after_gap {
                    self.line.clear();
                    self.skip_partial = true;
                }
                for &octet in &octets {
                    if self.skip_partial {
                        self.skip_partial = octet != b'\n';
                        continue;
                    }
                    self.line.push(octet);
                    if octet != b'\n' {
                        continue;
                    }
                    self.stats.lines += 1;
                    if let Some(logged) = line_ms(&self.line)
                        && logged >= since
                        && let Some(age) = board_ms.checked_sub(logged)
                    {
                        let half_trip = (self.last_round_trip / 2).as_millis() as u32;
                        self.stats.latencies_ms.push(age + half_trip);
                    }
                    let text = String::from_utf8_lossy(&self.line);
                    entries.push(Entry::Line(text.trim_end_matches(['\n', '\r']).to_owned()));
                    self.line.clear();
                }
            }
            LogPiece::Lost(octets) => {
                self.stats.lost += octets;
                entries.push(Entry::Lost(octets));
            }
            LogPiece::Restarted => {
                self.stats.restarts += 1;
                self.line.clear();
                self.skip_partial = false;
                self.reading_since_ms = None;
                entries.push(Entry::Restarted);
            }
            LogPiece::Boot(header) => {
                self.boot = Some(header);
                entries.push(Entry::Boot(header));
            }
            LogPiece::Dropped(lines) => {
                self.stats.dropped_lines += u64::from(lines);
                entries.push(Entry::Dropped(lines));
            }
        }
    }
}

/// A boot as its header tells it, in English: its ID, how long it has
/// run, why it began and how many boots began unexpectedly.
pub fn boot_text(header: &LogHeader) -> String {
    let mut text = format!("boot {:08x}, up {} s", header.boot, header.ms / 1000);
    match header.reset_reason {
        Some(reason) => {
            let name = reset_reason_name(reason).unwrap_or("reason");
            text.push_str(&format!(", reset {name} ({reason})"));
        }
        None => text.push_str(", reset reason not given"),
    }
    if let Some(resets) = header.resets {
        let plural = if resets == 1 { "" } else { "s" };
        text.push_str(&format!(", {resets} unexpected reset{plural}"));
    }
    text
}

/// A line's milliseconds since boot: ESP-IDF's `I (12345) tag: text`,
/// the first parenthesized number.
pub fn line_ms(line: &[u8]) -> Option<u32> {
    let open = line.iter().position(|&octet| octet == b'(')?;
    let rest = &line[open + 1..];
    let close = rest.iter().position(|&octet| octet == b')')?;
    std::str::from_utf8(&rest[..close]).ok()?.parse().ok()
}

/// Octets a frame of this payload takes on the wire: the Ethernet header
/// and the payload, padded to the least frame, without preamble or FCS.
fn wire_octets(payload: usize) -> u64 {
    (14 + payload).max(60) as u64
}

fn percentile(sorted: &[u32], share: f64) -> Option<u32> {
    let last = sorted.len().checked_sub(1)?;
    Some(sorted[((last as f64) * share).round() as usize])
}

/// Text in a JSON string, its quotes, backslashes and control characters
/// escaped.
pub fn json_text(text: &str) -> String {
    let mut out = String::new();
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            control if u32::from(control) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", u32::from(control)))
            }
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use atdecc::log::{LOG_ADDRESS, LogRing};

    #[test]
    fn a_boot_line_says_why_it_began_when_the_device_says() {
        let mut header = LogHeader {
            boot: 0x812c_21f8,
            ms: 181_400,
            ring: 1,
            ..LogHeader::default()
        };
        assert_eq!(
            boot_text(&header),
            "boot 812c21f8, up 181 s, reset reason not given"
        );
        header.reset_reason = Some(4);
        header.resets = Some(3);
        assert_eq!(
            boot_text(&header),
            "boot 812c21f8, up 181 s, reset PANIC (4), 3 unexpected resets"
        );
        header.reset_reason = Some(40);
        header.resets = Some(1);
        assert_eq!(
            boot_text(&header),
            "boot 812c21f8, up 181 s, reset reason (40), 1 unexpected reset"
        );
    }

    #[test]
    fn a_lines_milliseconds_are_its_first_parenthesized_number() {
        assert_eq!(line_ms(b"I (12345) avb: hello (7)\n"), Some(12345));
        assert_eq!(line_ms(b"no time here\n"), None);
        assert_eq!(line_ms(b"W (abc) x\n"), None);
    }

    #[test]
    fn a_line_after_a_gap_starts_at_the_next_newline() {
        let mut reader = Reader::new(LOG_ADDRESS, Instant::from_nanos(0));
        reader.last_round_trip = Duration::from_millis(2);
        reader.reading_since_ms = Some(90);
        let mut entries = Vec::new();
        reader.take(
            LogPiece::Text {
                octets: b"tail of a line\nI (100) a: whole\nI (1".to_vec(),
                board_ms: 150,
                after_gap: true,
            },
            &mut entries,
        );
        reader.take(
            LogPiece::Text {
                octets: b"20) a: split\nI (80) a: waiting\n".to_vec(),
                board_ms: 160,
                after_gap: false,
            },
            &mut entries,
        );
        assert_eq!(
            entries,
            [
                Entry::Line("I (100) a: whole".into()),
                Entry::Line("I (120) a: split".into()),
                Entry::Line("I (80) a: waiting".into()),
            ]
        );
        // 150 - 100 and 160 - 120, each plus half of a 2 ms round trip;
        // none for the line logged before reading began.
        assert_eq!(reader.stats.latencies_ms, [51, 41]);
        assert_eq!(reader.stats.lines, 3);
    }

    /// A device's ring, answering each read as the device would.
    fn answer(ring: &LogRing, reads: &[(u64, u16)], ms: u32) -> MemoryRead {
        let header = ring.header(ms, 0).encode();
        let mut tlvs = Vec::new();
        for &(address, length) in reads {
            let mut octets = Vec::new();
            ring.read(
                address - LOG_ADDRESS,
                usize::from(length),
                &header,
                &mut octets,
            )
            .unwrap();
            tlvs.push((address, octets));
        }
        MemoryRead {
            status: AaStatus::SUCCESS,
            tlvs,
        }
    }

    /// Followed through a device's own ring, a log reads as written: its
    /// boot first, then its lines, a restart and the next boot's.
    #[test]
    fn a_followed_log_reads_as_the_device_wrote_it() {
        let mut ring = LogRing::new(1024, 0x1111_2222);
        ring.set_reset(1, 0);
        ring.write(b"I (10) main: one\nI (20) main: two\n");
        let mut reader = Reader::new(LOG_ADDRESS, Instant::from_nanos(0));
        let mut entries = Vec::new();
        let mut at = 0u64;
        let mut step = |reader: &mut Reader, ring: &LogRing, entries: &mut Vec<Entry>| {
            at += 300_000_000;
            let now = Instant::from_nanos(at);
            if let Some(reads) = reader.poll_read(now) {
                reader.handle_read(now, &answer(ring, &reads, 1000), entries);
            }
        };
        for _ in 0..4 {
            step(&mut reader, &ring, &mut entries);
        }
        ring.write(b"I (30) main: three\n");
        for _ in 0..4 {
            step(&mut reader, &ring, &mut entries);
        }
        let mut next = LogRing::new(1024, 0x3333_4444);
        next.set_reset(4, 1);
        next.write(b"I (5) boot: again\n");
        for _ in 0..4 {
            step(&mut reader, &next, &mut entries);
        }
        let lines: Vec<&str> = entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Line(line) => Some(line.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(
            lines,
            [
                "I (10) main: one",
                "I (20) main: two",
                "I (30) main: three",
                "I (5) boot: again"
            ]
        );
        let boots: Vec<u32> = entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Boot(header) => Some(header.boot),
                _ => None,
            })
            .collect();
        assert_eq!(boots, [0x1111_2222, 0x3333_4444]);
        assert!(entries.contains(&Entry::Restarted));
        assert_eq!(reader.boot().map(|boot| boot.reset_reason), Some(Some(4)));
        assert_eq!(reader.stats().restarts, 1);
        assert!(reader.caught_up());
    }

    #[test]
    fn a_refused_read_is_counted() {
        let mut reader = Reader::new(LOG_ADDRESS, Instant::from_nanos(0));
        let now = Instant::from_nanos(1);
        let reads = reader.poll_read(now).unwrap();
        assert_eq!(reads[0], (LOG_ADDRESS, LogHeader::LEN as u16));
        let refused = MemoryRead {
            status: AaStatus::ADDRESS_INVALID,
            tlvs: Vec::new(),
        };
        let mut entries = Vec::new();
        reader.handle_read(now, &refused, &mut entries);
        assert!(entries.is_empty());
        assert_eq!(reader.stats().refused, 1);
        assert_eq!(reader.stats().reads, 1);
    }

    #[test]
    fn json_text_escapes_what_json_must() {
        assert_eq!(json_text("a\"b\\c\n"), "a\\\"b\\\\c\\u000a");
    }
}
