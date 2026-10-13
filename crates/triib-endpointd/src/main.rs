//! triib-endpointd: this computer's entities on an AVB network, each its
//! own ATDECC entity with streams that any controller can connect, with
//! audio from and to the computer's audio.
//!
//! It takes gPTP from ptp4l, which must run on the interface, and needs
//! `CAP_NET_RAW`. The devices are listed in `endpoints.toml` in triib's
//! data folder, written with one device the first time.

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use triib_endpointd::config::{self, Config, EndpointConfig};
use triib_endpointd::runtime;

const USAGE: &str = "usage: triib-endpointd [--config FILE] [--interface NAME]";

/// A first configuration: a device with a stream each way on the first
/// wired interface with a PTP hardware clock.
fn first_config() -> Config {
    let interface = avb_net::interfaces()
        .into_iter()
        .find(|interface| {
            interface.physical && !interface.wireless && interface.hardware_clock.is_some()
        })
        .map(|interface| interface.name)
        .unwrap_or_default();
    Config {
        interface,
        ptp4l_socket: "/var/run/ptp4lro".into(),
        endpoints: vec![EndpointConfig::new("Host entity 1", None)],
        ..Config::default()
    }
}

fn main() -> ExitCode {
    let mut arguments = std::env::args().skip(1);
    let mut path: Option<PathBuf> = None;
    let mut interface: Option<String> = None;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--config" => path = arguments.next().map(PathBuf::from),
            "--interface" => interface = arguments.next(),
            "--version" | "-V" => {
                println!("triib-endpointd {}", env!("CARGO_PKG_VERSION"));
                return ExitCode::SUCCESS;
            }
            _ => {
                eprintln!("{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    let Some(path) = path.or_else(config::path) else {
        eprintln!("no data folder for endpoints.toml; give one with --config");
        return ExitCode::FAILURE;
    };
    let mut config = if path.exists() {
        match triib_store::load::<Config>(&path) {
            Ok(config) => config,
            Err(error) => {
                eprintln!("{}: {error}", path.display());
                return ExitCode::FAILURE;
            }
        }
    } else {
        let config = first_config();
        if let Err(error) = triib_store::save(&path, &config) {
            eprintln!("{}: {error}", path.display());
            return ExitCode::FAILURE;
        }
        println!("wrote {}", path.display());
        config
    };
    if let Some(interface) = interface {
        config.interface = interface;
    }
    if config.interface.is_empty() {
        eprintln!(
            "no interface: name one in {} or with --interface",
            path.display()
        );
        return ExitCode::FAILURE;
    }
    // One daemon at a time: another would claim the same entity IDs.
    let _lock = match triib_endpointd::status::take_lock() {
        Ok(Some(lock)) => lock,
        Ok(None) => {
            eprintln!("triib-endpointd runs already");
            return ExitCode::FAILURE;
        }
        Err(error) => {
            eprintln!("could not take the daemon's lock: {error}");
            return ExitCode::FAILURE;
        }
    };
    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop = stop.clone();
        let _ = ctrlc::set_handler(move || stop.store(true, Ordering::Relaxed));
    }
    println!(
        "triib-endpointd {} on {}",
        env!("CARGO_PKG_VERSION"),
        config.interface
    );
    // One daemon on an interface, whoever runs it, claimed again when the
    // file names another.
    let mut claimed: Option<(String, triib_endpointd::status::Claim)> = None;
    loop {
        if claimed
            .as_ref()
            .is_none_or(|(interface, _)| *interface != config.interface)
        {
            // The interface it held before is free first.
            drop(claimed.take());
            match triib_endpointd::status::claim_interface(&config.interface) {
                Ok(Some(claim)) => claimed = Some((config.interface.clone(), claim)),
                Ok(None) => {
                    eprintln!(
                        "another triib-endpointd runs on {}, for this or another user or root",
                        config.interface
                    );
                    return ExitCode::FAILURE;
                }
                Err(error) => {
                    eprintln!("could not claim {}: {error}", config.interface);
                    return ExitCode::FAILURE;
                }
            }
        }
        match runtime::Runtime::new(config, Some(path.clone()), stop.clone())
            .and_then(runtime::Runtime::run)
        {
            Ok(runtime::Exit::Stop) => return ExitCode::SUCCESS,
            Ok(runtime::Exit::Reload(changed)) => config = changed,
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        }
    }
}
