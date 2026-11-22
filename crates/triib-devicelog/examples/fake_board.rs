//! A stand-in for an ESP32 board in a namespace's tests, never on a
//! network: an entity with no streams that keeps a log as the boards do,
//! a LOG_OBJECT memory object read through Address Access (see
//! `atdecc::log`), so `triib-cli log` and triib's device log can be tried without one. It
//! writes lines as ESP-IDF prints them, `I (ms) tag: text`.
//!
//!   fake_board --interface NAME [--name NAME] [--instance N] [--ring OCTETS]
//!              [--rate LINES_PER_SECOND] [--line OCTETS] [--burst OCTETS]
//!              [--restart-after SECONDS] [--drop-every N] [--for SECONDS]
//!              [--crash-dump OCTETS] [--crash-max OCTETS] [--crash-file PATH]
//!              [--reset-reason N] [--resets N]
//!
//! `--burst` writes that many octets of lines at once at the start, as a
//! boot does. `--restart-after` starts the log over in a new boot that
//! long after the start, once. `--drop-every` counts every Nth line as
//! dropped instead of writing it. It writes each line on stdout too, to
//! compare with what was read.
//!
//! `--crash-dump` keeps a crash dump of that many octets, as a board's
//! CRASH_DUMP memory object, which START_OPERATION erases; `--crash-max`
//! alone keeps an empty one, up to that size (256 KiB by default).
//! `--crash-file` writes the dump's octets there, to compare with what
//! was read.
//!
//! `--reset-reason` and `--resets` put why its first boot began and how
//! many began unexpectedly in its log's header, as an ESP32 board's; a
//! later boot, from `--restart-after`, begins for SW.

use std::io::Write;
use std::time::Duration;

use atdecc::entity::{ClockQuality, EndpointModel, Entity};
use atdecc::{ADP_ACMP_MULTICAST, ClockIdentity, EntityModelId, Instant};
use avb_net::Socket;

struct Options {
    interface: String,
    name: String,
    instance: u8,
    ring: u32,
    rate: f64,
    line: usize,
    burst: usize,
    restart_after: Option<Duration>,
    drop_every: Option<u64>,
    run_for: Option<Duration>,
    crash_dump: Option<usize>,
    crash_max: Option<u64>,
    crash_file: Option<String>,
    reset_reason: Option<u8>,
    resets: u16,
}

const USAGE: &str = "usage: fake_board --interface NAME [--name NAME] [--instance N] \
                     [--ring OCTETS] [--rate LINES_PER_SECOND] [--line OCTETS] \
                     [--burst OCTETS] [--restart-after SECONDS] [--drop-every N] \
                     [--for SECONDS] [--crash-dump OCTETS] [--crash-max OCTETS] \
                     [--crash-file PATH] [--reset-reason N] [--resets N]";

/// esp_reset_reason_t's SW.
const SW: u8 = 3;

fn parse(args: &[String]) -> Result<Options, String> {
    let mut options = Options {
        interface: String::new(),
        name: "fake-board".into(),
        instance: 0x30,
        ring: 32 * 1024,
        rate: 2.0,
        line: 60,
        burst: 0,
        restart_after: None,
        drop_every: None,
        run_for: None,
        crash_dump: None,
        crash_max: None,
        crash_file: None,
        reset_reason: None,
        resets: 0,
    };
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = || args.next().ok_or(format!("{arg} needs a value"));
        let number = |text: &String| {
            text.parse::<f64>()
                .map_err(|_| format!("{arg}: not a number"))
        };
        match arg.as_str() {
            "--interface" => options.interface = value()?.clone(),
            "--name" => options.name = value()?.clone(),
            "--instance" => options.instance = number(value()?)? as u8,
            "--ring" => options.ring = number(value()?)? as u32,
            "--rate" => options.rate = number(value()?)?,
            "--line" => options.line = number(value()?)? as usize,
            "--burst" => options.burst = number(value()?)? as usize,
            "--restart-after" => {
                options.restart_after = Some(Duration::from_secs_f64(number(value()?)?));
            }
            "--drop-every" => options.drop_every = Some(number(value()?)? as u64),
            "--for" => options.run_for = Some(Duration::from_secs_f64(number(value()?)?)),
            "--crash-dump" => options.crash_dump = Some(number(value()?)? as usize),
            "--crash-max" => options.crash_max = Some(number(value()?)? as u64),
            "--crash-file" => options.crash_file = Some(value()?.clone()),
            "--reset-reason" => options.reset_reason = Some(number(value()?)? as u8),
            "--resets" => options.resets = number(value()?)? as u16,
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    if options.interface.is_empty() {
        return Err("--interface is needed".into());
    }
    Ok(options)
}

fn model(options: &Options, mac: atdecc::MacAddress) -> EndpointModel {
    let [a, b, c, d, e, f] = mac.0;
    let entity_id = atdecc::EntityId(u64::from_be_bytes([
        a,
        b,
        c,
        0xff,
        options.instance,
        d,
        e,
        f,
    ]));
    let [a, b, c, d, e, f] = mac.0;
    EndpointModel {
        entity_id,
        entity_model_id: EntityModelId(0x8c1f_6436_c030_0002),
        entity_name: options.name.clone(),
        group_name: String::new(),
        vendor_name: "Scramble Tools".into(),
        model_name: "Fake board".into(),
        firmware_version: "0.0.1".into(),
        serial_number: String::new(),
        mac,
        interface_name: options.interface.clone(),
        clock_identity: ClockIdentity(u64::from_be_bytes([a, b, c, 0xff, 0xfe, d, e, f])),
        clock_quality: ClockQuality::grandmaster_capable(248),
        sampling_rates: vec![48_000],
        current_sampling_rate: 48_000,
        outputs: Vec::new(),
        inputs: Vec::new(),
        clock_source: 0,
        clock_source_inputs: Vec::new(),
        logo: None,
        vendor_logo: None,
    }
}

/// A line of `length` octets, newline included, logged at `ms`.
fn line(ms: u64, number: u64, length: usize) -> String {
    let mut text = format!("I ({ms}) fake: line {number} ");
    while text.len() + 1 < length {
        text.push(char::from(b'a' + (text.len() % 26) as u8));
    }
    text.push('\n');
    text
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let options = match parse(&args) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("{error}\n{USAGE}");
            std::process::exit(2);
        }
    };
    // Raw sockets through avb-endpointd's helper where it is installed,
    // Linux alone having one.
    #[cfg(target_os = "linux")]
    let _ = avb_net::broker::use_helper(
        "avb-netraw",
        &[std::path::Path::new("/usr/lib/avb-endpointd")],
    );
    let Some(mac) = avb_net::interfaces()
        .into_iter()
        .find(|found| found.name == options.interface)
        .map(|found| found.mac)
    else {
        eprintln!("no interface named {}", options.interface);
        std::process::exit(1);
    };
    if let Err(error) = run(&options, mac) {
        eprintln!("fake_board: {error}");
        std::process::exit(1);
    }
}

fn run(options: &Options, mac: atdecc::MacAddress) -> std::io::Result<()> {
    let socket = Socket::open(&options.interface, atdecc::ETHERTYPE_AVTP)?;
    socket.join_multicast(ADP_ACMP_MULTICAST)?;
    let started = std::time::Instant::now();
    let clock = || Instant::from_nanos(started.elapsed().as_nanos() as u64);
    let mut entity = Entity::new(model(options, mac), &[]);
    let mut boot = 1;
    // Lines count milliseconds from their boot, as a board's do.
    let mut booted = clock();
    entity.keep_log(booted, options.ring, boot);
    if let Some(reason) = options.reset_reason {
        entity.set_log_reset(reason, options.resets);
    }
    if options.crash_dump.is_some() || options.crash_max.is_some() {
        let dump = crash_dump(options.crash_dump.unwrap_or(0));
        if let Some(path) = &options.crash_file {
            std::fs::write(path, &dump)?;
        }
        entity.keep_crash_dump(options.crash_max.unwrap_or(256 * 1024), dump);
    }
    entity.start(clock());
    let mut stdout = std::io::stdout().lock();
    let mut number = 0;
    let mut write_line = |entity: &mut Entity, number: &mut u64, booted: Instant, now: Instant| {
        let ms = now.saturating_duration_since(booted).as_millis() as u64;
        *number += 1;
        if options
            .drop_every
            .is_some_and(|every| every > 0 && number.is_multiple_of(every))
        {
            entity.log_dropped_line();
            return;
        }
        let text = line(ms, *number, options.line);
        entity.log(text.as_bytes());
        let _ = stdout.write_all(text.as_bytes());
        let _ = stdout.flush();
    };
    // A boot's burst: lines enough for the octets asked, at once.
    while (number as usize) * options.line < options.burst {
        write_line(&mut entity, &mut number, booted, clock());
    }
    let every = Duration::from_secs_f64(1.0 / options.rate.max(0.001));
    let mut next_line = clock() + every;
    let mut restarted = false;
    let mut buffer = vec![0u8; 1600];
    loop {
        let now = clock();
        if options
            .run_for
            .is_some_and(|run_for| now.since_start() >= run_for)
        {
            entity.depart();
            while let Some((destination, frame)) = entity.poll_transmit() {
                let _ = socket.send(destination, &frame);
            }
            return Ok(());
        }
        if !restarted
            && let Some(after) = options.restart_after
            && now.since_start() >= after
        {
            restarted = true;
            boot += 1;
            // A new boot: a fresh log, from its own zero.
            entity.keep_log(now, options.ring, boot);
            if options.reset_reason.is_some() {
                entity.set_log_reset(SW, options.resets);
            }
            booted = now;
            number = 0;
        }
        entity.handle_timeout(now);
        if now >= next_line {
            next_line += every;
            if options.rate > 0.0 {
                write_line(&mut entity, &mut number, booted, now);
            }
        }
        while let Some((destination, frame)) = entity.poll_transmit() {
            let _ = socket.send(destination, &frame);
        }
        let wake = [entity.poll_timeout(), Some(next_line)]
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(now + Duration::from_millis(100));
        let wait = wake
            .saturating_duration_since(now)
            .min(Duration::from_millis(100));
        if let Ok(Some(received)) = socket.receive(&mut buffer, Some(wait)) {
            entity.handle_frame(clock(), received.source, &buffer[..received.length]);
        }
    }
}

/// A stand-in for a core dump of `length` octets: a name, then a pattern
/// that shows any octet out of place.
fn crash_dump(length: usize) -> Vec<u8> {
    let mut dump = b"FAKE CORE DUMP\n".to_vec();
    dump.extend((0..length).map(|at| (at.wrapping_mul(31) ^ (at >> 8)) as u8));
    dump.truncate(length);
    dump
}
