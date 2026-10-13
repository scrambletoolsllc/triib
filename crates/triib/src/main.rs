// Release builds on Windows open no console window.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod agents;
mod app;
mod column_resize;
mod control;
mod controls_view;
mod describe;
mod diagnostics_view;
mod dialog;
mod entity_table;
mod header_band;
mod host;
mod host_view;
mod i18n;
mod lite_view;
mod log_view;
mod mapping_view;
mod matrix;
mod mcp;
mod netmap;
mod network;
mod presets;
mod settings;
mod settings_view;
mod storage;
mod text;
mod tools;
mod topology;
mod vendor;
mod view;
mod wireless_view;

use std::sync::{Mutex, OnceLock};

use iced::futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use scramble_ui::{font, omarchy};

/// Events from background threads, delivered through a subscription.
#[derive(Debug, Clone)]
pub enum External {
    OmarchyThemeChanged,
    Interfaces(Vec<avb_net::Interface>),
    Network(network::Report),
    /// What triib-endpointd says it is doing, or `None` when it is not
    /// running.
    Endpoints(Option<triib_endpointd::status::DaemonStatus>),
    /// The interface another user's or root's triib-endpointd holds, the
    /// one this user's endpoints would run on.
    EndpointsElsewhere(Option<String>),
    /// The audio devices' names: inputs, then outputs.
    AudioDevices(Vec<String>, Vec<String>),
    /// A request on the control channel, for `triib --mcp` and other
    /// programs.
    Control(control::Call),
}

static EXTERNAL_EVENTS: Mutex<Option<UnboundedReceiver<External>>> = Mutex::new(None);
static EXTERNAL_SENDER: OnceLock<UnboundedSender<External>> = OnceLock::new();

/// Sends `event` to the app from any thread.
pub fn post(event: External) {
    if let Some(sender) = EXTERNAL_SENDER.get() {
        let _ = sender.unbounded_send(event);
    }
}

/// How often interfaces are looked at again, for links going up or down
/// and adapters plugged in.
const INTERFACE_POLL: std::time::Duration = std::time::Duration::from_secs(2);

fn main() -> iced::Result {
    if let Some(argument) = std::env::args().nth(1) {
        match argument.as_str() {
            "--version" | "-V" => println!("triib {}", env!("CARGO_PKG_VERSION")),
            "--mcp" => {
                if let Err(error) = mcp::run() {
                    eprintln!("triib --mcp: {error}");
                    std::process::exit(1);
                }
            }
            _ => println!("usage: triib [--version | --mcp]"),
        }
        return Ok(());
    }

    scramble_ui::input::remember_interface_thread();
    let (sender, receiver) = mpsc::unbounded();
    let _ = EXTERNAL_SENDER.set(sender);
    *EXTERNAL_EVENTS
        .lock()
        .unwrap_or_else(|error| error.into_inner()) = Some(receiver);

    let omarchy_dir = omarchy::current_theme_dir();
    if let Some(dir) = omarchy_dir.clone() {
        omarchy::watch(dir, || post(External::OmarchyThemeChanged));
    }
    watch_interfaces();
    serve_control();
    if host::SUPPORTED {
        watch_endpoints();
    }

    iced::application(
        move || app::Triib::boot(omarchy_dir.clone()),
        app::Triib::update,
        app::Triib::view,
    )
    .settings(iced::Settings {
        id: Some(app::APP_ID.to_owned()),
        fonts: font::files().collect(),
        default_font: font::TEXT,
        default_text_size: font::DEFAULT_SIZE.into(),
        antialiasing: true,
        ..iced::Settings::default()
    })
    .window(iced::window::Settings {
        size: iced::Size::new(1280.0, 800.0),
        icon: window_icon(),
        min_size: Some(iced::Size::new(360.0, 480.0)),
        // On Linux, the app id desktops match windows and launchers by.
        #[cfg(target_os = "linux")]
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: app::APP_ID.to_owned(),
            ..Default::default()
        },
        ..iced::window::Settings::default()
    })
    .title(app::Triib::title)
    .theme(app::Triib::theme)
    .subscription(app::Triib::subscription)
    .run()
}

/// Opens the control channel, whose calls the app answers; triib runs
/// without it where it cannot be opened.
fn serve_control() {
    let opened = control::address()
        .and_then(|address| control::serve(address, |call| post(External::Control(call))));
    if let Err(error) = opened {
        eprintln!("triib: running without the control channel: {error}");
    }
}

/// The app's icon on its windows, where the desktop takes it from them,
/// as X11 and Windows do; Wayland desktops take it from the desktop entry.
fn window_icon() -> Option<iced::window::Icon> {
    let file: &[u8] = include_bytes!(
        "../../../data/icons/hicolor/256x256/apps/io.github.scrambletools.triib.png"
    );
    let mut reader = png::Decoder::new(std::io::Cursor::new(file))
        .read_info()
        .ok()?;
    let mut rgba = vec![0; reader.output_buffer_size()?];
    let frame = reader.next_frame(&mut rgba).ok()?;
    if frame.color_type != png::ColorType::Rgba || frame.bit_depth != png::BitDepth::Eight {
        return None;
    }
    rgba.truncate(frame.buffer_size());
    iced::window::icon::from_rgba(rgba, frame.width, frame.height).ok()
}

/// Posts the interface list whenever it changes.
fn watch_interfaces() {
    let _ = std::thread::Builder::new()
        .name("interface-watch".into())
        .spawn(|| {
            let mut last = Vec::new();
            loop {
                let now = avb_net::interfaces();
                if now != last {
                    post(External::Interfaces(now.clone()));
                    last = now;
                }
                std::thread::sleep(INTERFACE_POLL);
            }
        });
}

/// Posts what triib-endpointd says whenever it changes, and the audio
/// devices now and then.
fn watch_endpoints() {
    let _ = std::thread::Builder::new()
        .name("endpoint-watch".into())
        .spawn(|| {
            let mut last = None;
            let mut last_elsewhere = None;
            let mut devices = None;
            let mut turn = 0u64;
            loop {
                turn += 1;
                let now = triib_endpointd::status::read();
                let recalling = host::RECALLING.load(std::sync::atomic::Ordering::Relaxed);
                // Without a daemon of this user's, whether another holds
                // the interface its endpoints would run on.
                let elsewhere = if now.is_none() {
                    host::elsewhere()
                } else {
                    None
                };
                if recalling || Some(&now) != last.as_ref() {
                    post(External::Endpoints(now.clone()));
                    last = Some(now);
                }
                if Some(&elsewhere) != last_elsewhere.as_ref() {
                    post(External::EndpointsElsewhere(elsewhere.clone()));
                    last_elsewhere = Some(elsewhere);
                }
                if turn % 15 == 1 {
                    let found = triib_stream::audio::device_names();
                    if Some(&found) != devices.as_ref() {
                        post(External::AudioDevices(found.0.clone(), found.1.clone()));
                        devices = Some(found);
                    }
                }
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        });
}

/// Hands the receiver to the subscription once.
fn external_events() -> impl iced::futures::Stream<Item = External> {
    let receiver = EXTERNAL_EVENTS
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .take();
    iced::futures::StreamExt::flatten(iced::futures::stream::iter(receiver))
}

#[cfg(test)]
mod icon_tests {
    #[test]
    fn the_window_icon_decodes() {
        assert!(super::window_icon().is_some());
    }
}
