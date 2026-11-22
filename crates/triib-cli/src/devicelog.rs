//! `triib-cli log` and `triib-cli crash`: the log a device keeps in a
//! LOG_OBJECT memory object, printed as it is read, and the crash dump it
//! keeps in a CRASH_DUMP one, saved to a file (see [`triib_devicelog`]).
//! Each reads as a controller of its own, FF-FD in its entity ID as the
//! other commands, and reads only the models of the devices it looks
//! for: the one named by ID, or, for a name, those that advertise Address
//! Access. A device is asked nothing unless one of them runs.
//!
//! `log` says in a line of its own, before a boot's lines and after each
//! restart, the boot's ID, how long it has run, and, where the device
//! says, why it began and how many boots began unexpectedly:
//! `[esp-mic: boot 812c21f8, up 181 s, reset PANIC (4), 3 unexpected
//! resets]`. With `--stats`, it says on stderr every 10 s, and at the
//! end, what reading cost; `--stats-json FILE` writes the same as JSON,
//! and `--poll MS` reads octets every MS milliseconds, never the header
//! alone: the most reading costs.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use atdecc::adp::EntityCapabilities;
use atdecc::aecp::{AddressAccessPdu, AecpHeader, AecpMessageType};
use atdecc::blocking::Driver;
use atdecc::controller::{CommandId, Event, Outcome};
use atdecc::descriptor::{DescriptorType, MemoryObjectOperation};
use atdecc::{EntityId, MacAddress};
use triib_devicelog::crash::{self, CrashDump, DumpRead};
use triib_devicelog::log::{self, Entry, Reader};

/// How long to look for the devices asked for.
const FIND_FOR: Duration = Duration::from_secs(10);
const STATS_EVERY: Duration = Duration::from_secs(10);
/// How long a command may take, its retry included.
const ANSWER_WITHIN: Duration = Duration::from_secs(2);

/// A device asked for: by entity ID, or by entity name.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Wanted {
    Id(EntityId),
    Name(String),
}

impl Wanted {
    fn of(text: &str) -> Self {
        text.parse()
            .map_or_else(|_| Wanted::Name(text.to_owned()), Wanted::Id)
    }
}

impl std::fmt::Display for Wanted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wanted::Id(id) => write!(f, "{id}"),
            Wanted::Name(name) => write!(f, "\"{name}\""),
        }
    }
}

/// Opens `interface` as a controller that reads only the entities asked
/// for, and only their descriptors.
fn open(interface: &str) -> Result<Driver, String> {
    Driver::open_with(interface, |config| {
        config.register_unsolicited = false;
        config.enumerate = false;
        let mut octets = config.entity_id.0.to_be_bytes();
        octets[4] = 0xfd;
        config.entity_id = EntityId(u64::from_be_bytes(octets));
    })
    .map_err(|error| format!("{interface}: {error}"))
}

/// The label of a device in what is printed: its name, else its ID.
fn label_of(name: &str, id: EntityId) -> String {
    if name.is_empty() {
        id.to_string()
    } else {
        name.to_owned()
    }
}

/// The sequence ID of an Address Access command (or response) between
/// `controller` and a device.
fn address_access(bytes: &[u8], controller: EntityId, response: bool) -> Option<u16> {
    let (header, _) = AecpHeader::decode(bytes).ok()?;
    let wanted = if response {
        AecpMessageType::ADDRESS_ACCESS_RESPONSE
    } else {
        AecpMessageType::ADDRESS_ACCESS_COMMAND
    };
    (header.message_type == wanted && header.controller_entity_id == controller)
        .then(|| AddressAccessPdu::decode(bytes).ok())
        .flatten()
        .map(|pdu| pdu.header.sequence_id)
}

pub struct LogOptions {
    entities: Vec<Wanted>,
    follow: bool,
    stats: bool,
    stats_json: Option<PathBuf>,
    poll: Option<Duration>,
}

impl LogOptions {
    pub fn parse(words: &[String]) -> Result<Self, String> {
        let mut options = LogOptions {
            entities: Vec::new(),
            follow: false,
            stats: false,
            stats_json: None,
            poll: None,
        };
        let mut words = words.iter();
        while let Some(word) = words.next() {
            match word.as_str() {
                "--follow" | "-f" => options.follow = true,
                "--stats" => options.stats = true,
                "--stats-json" => {
                    let file = words.next().ok_or("--stats-json needs a file")?;
                    options.stats_json = Some(PathBuf::from(file));
                }
                "--poll" => {
                    let millis: u64 = words
                        .next()
                        .and_then(|millis| millis.parse().ok())
                        .filter(|&millis| millis > 0)
                        .ok_or("--poll needs milliseconds, more than 0")?;
                    options.poll = Some(Duration::from_millis(millis));
                }
                flag if flag.starts_with('-') => return Err(format!("no option {flag} for log")),
                text => options.entities.push(Wanted::of(text)),
            }
        }
        if options.entities.is_empty() {
            return Err("log needs a device: its entity ID or its name".into());
        }
        Ok(options)
    }
}

/// A device whose log is followed.
struct Board {
    wanted: Wanted,
    entity_id: Option<EntityId>,
    mac: MacAddress,
    name: String,
    reader: Option<Reader>,
    out: Option<CommandId>,
    /// When each Address Access command with this sequence ID went.
    sent: BTreeMap<u16, SystemTime>,
}

impl Board {
    fn label(&self) -> String {
        self.entity_id
            .map_or_else(|| self.wanted.to_string(), |id| label_of(&self.name, id))
    }

    fn print(&self, entry: &Entry, prefix: bool, out: &mut impl Write) {
        let label = self.label();
        let _ = match entry {
            Entry::Line(line) if prefix => writeln!(out, "{label}: {line}"),
            Entry::Line(line) => writeln!(out, "{line}"),
            Entry::Boot(header) => writeln!(out, "[{label}: {}]", log::boot_text(header)),
            Entry::Restarted => writeln!(out, "[{label} restarted]"),
            Entry::Lost(octets) => writeln!(
                out,
                "[{label}: {octets} bytes lost, written faster than read]"
            ),
            Entry::Dropped(lines) => writeln!(out, "[{label} dropped {lines} lines]"),
        };
    }

    fn stats_json(&self, elapsed: Duration) -> Option<String> {
        let reader = self.reader.as_ref()?;
        let entity = self.entity_id.map_or_else(String::new, |id| id.to_string());
        Some(
            reader
                .stats()
                .json(&entity, &self.name, reader.boot(), elapsed),
        )
    }
}

fn write_json(path: &Path, boards: &[Board], elapsed: Duration) {
    let entries: Vec<String> = boards
        .iter()
        .filter_map(|board| board.stats_json(elapsed))
        .collect();
    let json = format!("{{\"boards\":[{}]}}\n", entries.join(","));
    let temporary = path.with_extension("json.part");
    if std::fs::write(&temporary, json).is_ok() {
        let _ = std::fs::rename(&temporary, path);
    }
}

/// Prints the logs of the devices asked for, until each has been read to
/// its end, or with `--follow` until stopped.
pub fn log(interface: &str, options: &LogOptions) -> Result<(), String> {
    let mut driver = open(interface)?;
    driver.keep_frames(4096);
    let controller_id = driver.controller().entity_id();
    let started = std::time::Instant::now();
    let mut boards: Vec<Board> = options
        .entities
        .iter()
        .map(|wanted| Board {
            wanted: wanted.clone(),
            entity_id: None,
            mac: MacAddress([0; 6]),
            name: String::new(),
            reader: None,
            out: None,
            sent: BTreeMap::new(),
        })
        .collect();
    if boards
        .iter()
        .any(|board| matches!(board.wanted, Wanted::Name(_)))
    {
        driver.controller_mut().discover(None);
    } else {
        for board in &boards {
            if let Wanted::Id(id) = board.wanted {
                driver.controller_mut().discover(Some(id));
            }
        }
    }
    let prefix = boards.len() > 1;
    let mut next_stats = started + STATS_EVERY;
    let mut stdout = std::io::stdout().lock();
    let mut entries = Vec::new();

    loop {
        let now = driver.now();
        for board in boards.iter_mut().filter(|board| board.out.is_none()) {
            let (Some(id), Some(reader)) = (board.entity_id, board.reader.as_mut()) else {
                continue;
            };
            if let Some(reads) = reader.poll_read(now) {
                board.out = Some(driver.controller_mut().read_memory(now, id, &reads));
            }
        }
        let wait = boards
            .iter()
            .filter(|board| board.out.is_none())
            .filter_map(|board| board.reader.as_ref()?.poll_timeout())
            .min()
            .map_or(Duration::from_millis(100), |due| {
                due.saturating_duration_since(now)
            })
            .min(Duration::from_millis(100));
        driver
            .turn(wait)
            .map_err(|error| format!("{interface}: {error}"))?;

        while let Some(frame) = driver.poll_frame() {
            let Some(board) = boards
                .iter_mut()
                .find(|board| board.entity_id.is_some() && board.mac == frame.peer)
            else {
                continue;
            };
            let Some(sequence) = address_access(&frame.bytes, controller_id, !frame.sent) else {
                continue;
            };
            let Some(reader) = board.reader.as_mut() else {
                continue;
            };
            if frame.sent {
                reader.sent(frame.bytes.len());
                board.sent.insert(sequence, frame.at);
            } else {
                let round_trip = board
                    .sent
                    .remove(&sequence)
                    .and_then(|sent| frame.at.duration_since(sent).ok());
                reader.received(frame.bytes.len(), round_trip);
            }
        }

        while let Some(event) = driver.controller_mut().poll_event() {
            let now = driver.now();
            match event {
                Event::EntityOnline(id) => {
                    let Some(entity) = driver.controller().entity(id) else {
                        continue;
                    };
                    let mac = entity.mac;
                    let address_access = entity
                        .adp
                        .entity_capabilities
                        .contains(EntityCapabilities::ADDRESS_ACCESS_SUPPORTED);
                    let mut read = false;
                    for board in boards.iter_mut() {
                        match &board.wanted {
                            Wanted::Id(wanted) if *wanted == id => read = true,
                            Wanted::Name(_) if board.entity_id.is_none() && address_access => {
                                read = true
                            }
                            _ => {}
                        }
                        if board.entity_id == Some(id) {
                            board.mac = mac;
                            if let Some(reader) = &mut board.reader {
                                reader.resume(now);
                                let _ = writeln!(stdout, "[{} is back]", board.label());
                            }
                        }
                    }
                    if read && driver.controller().model(id).is_none() {
                        driver.controller_mut().enumerate(now, id);
                    }
                }
                Event::EntityOffline(id, _) => {
                    for board in boards
                        .iter_mut()
                        .filter(|board| board.entity_id == Some(id))
                    {
                        if let Some(reader) = &mut board.reader {
                            reader.pause();
                            board.out = None;
                            let _ = writeln!(stdout, "[{} went away]", board.label());
                        }
                    }
                }
                Event::EntityEnumerated(id) => {
                    let controller = driver.controller();
                    let Some(model) = controller.model(id) else {
                        continue;
                    };
                    let name = model.entity_name().unwrap_or_default().to_owned();
                    let base = log::log_object(model);
                    let mac = controller.entity(id).map(|entity| entity.mac);
                    let taken: Vec<EntityId> =
                        boards.iter().filter_map(|board| board.entity_id).collect();
                    for board in boards.iter_mut().filter(|board| board.entity_id.is_none()) {
                        let wanted = match &board.wanted {
                            Wanted::Id(wanted) => *wanted == id,
                            Wanted::Name(wanted) => *wanted == name && !taken.contains(&id),
                        };
                        if !wanted {
                            continue;
                        }
                        let Some(base) = base else {
                            return Err(format!(
                                "{} keeps no log (no LOG_OBJECT memory object)",
                                label_of(&name, id)
                            ));
                        };
                        board.entity_id = Some(id);
                        board.mac = mac.unwrap_or(board.mac);
                        board.name = name.clone();
                        let mut reader = Reader::new(base, now);
                        if let Some(poll) = options.poll {
                            reader.poll_every(poll);
                        }
                        board.reader = Some(reader);
                        break;
                    }
                }
                Event::CommandFinished(command, outcome) => {
                    let Some(board) = boards.iter_mut().find(|board| board.out == Some(command))
                    else {
                        continue;
                    };
                    board.out = None;
                    let answer = (outcome == Outcome::Done)
                        .then(|| driver.controller_mut().take_memory_read(command))
                        .flatten();
                    let Some(reader) = &mut board.reader else {
                        continue;
                    };
                    match answer {
                        Some(answer) => reader.handle_read(now, &answer, &mut entries),
                        None => reader.handle_failure(now, outcome == Outcome::NoResponse),
                    }
                    for entry in entries.drain(..) {
                        board.print(&entry, prefix, &mut stdout);
                    }
                }
                _ => {}
            }
        }
        let _ = stdout.flush();

        let finding = boards.iter().any(|board| board.reader.is_none());
        let elapsed = started.elapsed();
        if finding && elapsed >= FIND_FOR {
            let missing: Vec<String> = boards
                .iter()
                .filter(|board| board.reader.is_none())
                .map(|board| board.wanted.to_string())
                .collect();
            return Err(format!(
                "no device {} found on {interface} with a log",
                missing.join(", ")
            ));
        }
        if !options.follow
            && !finding
            && boards
                .iter()
                .all(|board| board.reader.as_ref().is_some_and(Reader::caught_up))
        {
            break;
        }
        if std::time::Instant::now() >= next_stats {
            next_stats += STATS_EVERY;
            if options.stats {
                for board in &boards {
                    if let Some(reader) = &board.reader {
                        eprintln!("{}", reader.stats().line(&board.label(), elapsed));
                    }
                }
            }
            if let Some(path) = &options.stats_json {
                write_json(path, &boards, elapsed);
            }
        }
    }
    for board in &mut boards {
        let label = board.label();
        if let Some(line) = board.reader.as_mut().and_then(Reader::finish) {
            let _ = if prefix {
                writeln!(stdout, "{label}: {line}")
            } else {
                writeln!(stdout, "{line}")
            };
        }
    }
    let _ = stdout.flush();
    let elapsed = started.elapsed();
    if options.stats {
        for board in &boards {
            if let Some(reader) = &board.reader {
                eprintln!("{}", reader.stats().line(&board.label(), elapsed));
            }
        }
    }
    if let Some(path) = &options.stats_json {
        write_json(path, &boards, elapsed);
    }
    driver
        .close()
        .map_err(|error| format!("{interface}: {error}"))
}

pub struct CrashOptions {
    entity: Wanted,
    output: Option<PathBuf>,
    erase: bool,
}

impl CrashOptions {
    pub fn parse(words: &[String]) -> Result<Self, String> {
        let mut entity = None;
        let mut output = None;
        let mut erase = false;
        let mut words = words.iter();
        while let Some(word) = words.next() {
            match word.as_str() {
                "-o" | "--output" => {
                    let file = words.next().ok_or("-o needs a file")?;
                    output = Some(PathBuf::from(file));
                }
                "--erase" => erase = true,
                flag if flag.starts_with('-') => {
                    return Err(format!("no option {flag} for crash"));
                }
                text if entity.is_none() => entity = Some(Wanted::of(text)),
                text => return Err(format!("crash takes one device, not {text} too")),
            }
        }
        let entity = entity.ok_or("crash needs a device: its entity ID or its name")?;
        Ok(CrashOptions {
            entity,
            output,
            erase,
        })
    }
}

/// Runs the driver until `command` finishes, `within` at most.
fn wait(driver: &mut Driver, command: CommandId, within: Duration) -> Result<Outcome, String> {
    let until = std::time::Instant::now() + within;
    loop {
        driver
            .turn(Duration::from_millis(20))
            .map_err(|error| error.to_string())?;
        while let Some(event) = driver.controller_mut().poll_event() {
            if let Event::CommandFinished(finished, outcome) = event
                && finished == command
            {
                return Ok(outcome);
            }
        }
        if std::time::Instant::now() >= until {
            return Ok(Outcome::NoResponse);
        }
    }
}

/// Finds the device asked for and reads its model.
fn find(driver: &mut Driver, wanted: &Wanted, within: Duration) -> Result<EntityId, String> {
    let until = std::time::Instant::now() + within;
    driver.controller_mut().discover(match wanted {
        Wanted::Id(id) => Some(*id),
        Wanted::Name(_) => None,
    });
    let mut asked: Vec<EntityId> = Vec::new();
    loop {
        driver
            .turn(Duration::from_millis(50))
            .map_err(|error| error.to_string())?;
        while let Some(event) = driver.controller_mut().poll_event() {
            let now = driver.now();
            match event {
                Event::EntityOnline(id) => {
                    let Some(entity) = driver.controller().entity(id) else {
                        continue;
                    };
                    let read = match wanted {
                        Wanted::Id(wanted) => *wanted == id,
                        Wanted::Name(_) => entity
                            .adp
                            .entity_capabilities
                            .contains(EntityCapabilities::ADDRESS_ACCESS_SUPPORTED),
                    };
                    if read && !asked.contains(&id) {
                        asked.push(id);
                        driver.controller_mut().enumerate(now, id);
                    }
                }
                Event::EntityEnumerated(id) => {
                    let name = driver
                        .controller()
                        .model(id)
                        .and_then(|model| model.entity_name());
                    let found = match wanted {
                        Wanted::Id(wanted) => *wanted == id,
                        Wanted::Name(wanted) => name == Some(wanted.as_str()),
                    };
                    if found {
                        return Ok(id);
                    }
                }
                _ => {}
            }
        }
        if std::time::Instant::now() >= until {
            return Err(format!("no device {wanted} found"));
        }
    }
}

/// Reads the device's CRASH_DUMP descriptor again, for its length now.
fn refresh(driver: &mut Driver, id: EntityId, index: u16) -> Result<CrashDump, String> {
    let now = driver.now();
    let command =
        driver
            .controller_mut()
            .read_descriptor(now, id, DescriptorType::MEMORY_OBJECT, index);
    match wait(driver, command, ANSWER_WITHIN)? {
        Outcome::Done => driver
            .controller()
            .model(id)
            .and_then(crash::crash_dump)
            .ok_or_else(|| "its crash dump went away".into()),
        outcome => Err(format!("its crash dump's length: {outcome:?}")),
    }
}

/// Saves the crash dump the device keeps, and with `--erase` erases it.
pub fn crash(interface: &str, options: &CrashOptions) -> Result<(), String> {
    let mut driver = open(interface)?;
    let id = find(&mut driver, &options.entity, FIND_FOR)
        .map_err(|error| format!("{error} on {interface}"))?;
    let model = driver.controller().model(id).ok_or("its model went away")?;
    let name = label_of(model.entity_name().unwrap_or_default(), id);
    let firmware = model
        .entity()
        .map(|entity| entity.firmware_version.to_owned())
        .unwrap_or_default();
    let Some(object) = crash::crash_dump(model) else {
        return Err(format!(
            "{name} keeps no crash dump (no CRASH_DUMP memory object)"
        ));
    };
    let object = refresh(&mut driver, id, object.index)?;
    if object.length == 0 {
        println!("{name}: no crash dump");
        return Ok(());
    }
    let mut read = DumpRead::new(object).map_err(|failure| failure.to_string())?;
    let mut shown = 0;
    while let Some((address, length)) = read.next() {
        let now = driver.now();
        let command = driver
            .controller_mut()
            .read_memory(now, id, &[(address, length)]);
        let outcome = wait(&mut driver, command, ANSWER_WITHIN)?;
        let answer = (outcome == Outcome::Done)
            .then(|| driver.controller_mut().take_memory_read(command))
            .flatten();
        read.handle(outcome, answer.as_ref())
            .map_err(|failure| format!("{name}: {failure}"))?;
        let (done, total) = read.progress();
        let tenth = done * 10 / total;
        if tenth > shown && done < total {
            shown = tenth;
            eprintln!("{name}: {done} of {total} bytes read");
        }
    }
    let dump = read.into_octets();
    let path = options
        .output
        .clone()
        .unwrap_or_else(|| PathBuf::from(crash::file_name(id)));
    std::fs::write(&path, &dump).map_err(|error| format!("{}: {error}", path.display()))?;
    println!(
        "{name}: crash dump of {} bytes saved to {}",
        dump.len(),
        path.display()
    );
    println!("{name} runs firmware {firmware} now; the dump names the ELF it crashed in");
    println!("decode: {}", crash::decode_hint(&path));
    if options.erase {
        erase(&mut driver, id, &object)?;
        println!("{name}: crash dump erased");
    }
    driver
        .close()
        .map_err(|error| format!("{interface}: {error}"))
}

fn erase(driver: &mut Driver, id: EntityId, object: &CrashDump) -> Result<(), String> {
    let now = driver.now();
    let command = driver.controller_mut().start_operation(
        now,
        id,
        DescriptorType::MEMORY_OBJECT,
        object.index,
        MemoryObjectOperation::ERASE.0,
        &[],
    );
    match wait(driver, command, ANSWER_WITHIN)? {
        Outcome::Done => {}
        outcome => return Err(format!("erasing its crash dump: {outcome:?}")),
    }
    let after = refresh(driver, id, object.index)?;
    if after.length != 0 {
        return Err(format!(
            "its crash dump still holds {} bytes after the erase",
            after.length
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(text: &str) -> Vec<String> {
        text.split_whitespace().map(str::to_owned).collect()
    }

    #[test]
    fn log_options_parse() {
        let options = LogOptions::parse(&words(
            "esp-mic --follow --stats --poll 10 0x001b92fffe01b930",
        ))
        .unwrap();
        assert_eq!(
            options.entities,
            [
                Wanted::Name("esp-mic".into()),
                Wanted::Id(EntityId(0x001b_92ff_fe01_b930))
            ]
        );
        assert!(options.follow && options.stats);
        assert_eq!(options.poll, Some(Duration::from_millis(10)));
        assert!(LogOptions::parse(&[]).is_err());
        assert!(LogOptions::parse(&words("x --poll 0")).is_err());
        assert!(LogOptions::parse(&words("x --nope")).is_err());
    }

    #[test]
    fn crash_options_parse() {
        let options = CrashOptions::parse(&words("esp-mic -o mic.core --erase")).unwrap();
        assert_eq!(options.entity, Wanted::Name("esp-mic".into()));
        assert_eq!(options.output, Some(PathBuf::from("mic.core")));
        assert!(options.erase);
        assert!(CrashOptions::parse(&words("")).is_err());
        assert!(CrashOptions::parse(&words("a b")).is_err());
        assert!(CrashOptions::parse(&words("a --nope")).is_err());
        assert!(CrashOptions::parse(&words("a -o")).is_err());
    }

    #[test]
    fn a_boot_is_printed_in_brackets_with_its_device() {
        let board = Board {
            wanted: Wanted::Name("esp-mic".into()),
            entity_id: Some(EntityId(1)),
            mac: MacAddress([0; 6]),
            name: "esp-mic".into(),
            reader: None,
            out: None,
            sent: BTreeMap::new(),
        };
        let mut out = Vec::new();
        let header = log::LogHeader {
            boot: 0x812c_21f8,
            ms: 181_400,
            ring: 1,
            reset_reason: Some(4),
            resets: Some(3),
            ..log::LogHeader::default()
        };
        board.print(&Entry::Boot(header), false, &mut out);
        board.print(&Entry::Line("I (5) a: b".into()), true, &mut out);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "[esp-mic: boot 812c21f8, up 181 s, reset PANIC (4), 3 unexpected resets]\n\
             esp-mic: I (5) a: b\n"
        );
    }
}
