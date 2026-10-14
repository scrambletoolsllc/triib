//! Listening on this computer to an audio input, such as the Mac's own
//! AVB entity's, through one of its outputs. The audio runs in a helper,
//! `triib --monitor`, which the app starts again when it ends or stops
//! reporting: a Core Audio call can block for good when the AVB device
//! goes, and only a new process gets out of it.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError, mpsc};
use std::time::{Duration, Instant};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use triib_stream::audio::{Input, Output, Sink, Source};

/// The flag that runs this binary as the helper.
pub const FLAG: &str = "--monitor";

/// The rate the helper moves audio at, between the two devices' own.
const RATE: u32 = 48_000;
/// Frames moved at a time: 5 ms.
const CHUNK: usize = 240;
/// How often the helper reports, and looks at its devices.
const REPORT: Duration = Duration::from_secs(1);
/// How long without a report before the helper counts as hung.
const HUNG: Duration = Duration::from_secs(5);
/// How long the helper waits for a device that is not there, and the app
/// before starting a helper again.
const RETRY: Duration = Duration::from_secs(1);
const RESTART: Duration = Duration::from_secs(3);
/// Reads in a second while the input had nothing, past which it counts
/// as stalled: the device went, or stopped delivering.
const STALLED: u64 = 100;

/// What to listen to, and where: kept in the settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct Listen {
    /// Listening, or not.
    pub enabled: bool,
    /// The audio input, by name, as list_audio_devices names them.
    pub input: Option<String>,
    /// The input's first channel listened to, from 0.
    pub first_channel: u16,
    /// Channels listened to: 1, played on both sides, or 2.
    pub channels: u16,
    /// The audio output, by name; the default output when not given.
    pub output: Option<String>,
    /// Gain in decibels, from -60 to 24.
    pub gain_db: f32,
}

impl Default for Listen {
    fn default() -> Self {
        Self {
            enabled: false,
            input: None,
            first_channel: 0,
            channels: 2,
            output: None,
            gain_db: 0.0,
        }
    }
}

impl Listen {
    /// The same, its values within their bounds.
    pub fn bounded(mut self) -> Self {
        self.channels = self.channels.clamp(1, 2);
        self.gain_db = if self.gain_db.is_finite() {
            self.gain_db.clamp(-60.0, 24.0)
        } else {
            0.0
        };
        self
    }
}

/// What the helper is doing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state", content = "detail")]
pub enum State {
    /// Not listening.
    Off,
    /// Starting, or starting again after the helper ended or hung.
    Starting,
    /// Waiting for the input to be there, as the Mac's AVB device is only
    /// while one of its streams is connected.
    WaitingForInput(String),
    WaitingForOutput(String),
    /// Playing the input through the output.
    Playing,
}

/// What the helper last said.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    #[serde(flatten)]
    pub state: State,
    /// The loudest sample played in the last second, from 0 to 1.
    pub peak: f32,
}

impl Status {
    pub fn off() -> Self {
        Self {
            state: State::Off,
            peak: 0.0,
        }
    }
}

/// Runs as the helper with the arguments after the flag: the settings as
/// JSON. Plays until killed.
pub fn serve(arguments: &[String]) -> i32 {
    let Some(listen) = arguments
        .first()
        .and_then(|json| serde_json::from_str::<Listen>(json).ok())
        .map(Listen::bounded)
    else {
        eprintln!("usage: triib {FLAG} SETTINGS-AS-JSON");
        return 2;
    };
    loop {
        let source = Source::Device {
            name: listen.input.clone(),
            first_channel: listen.first_channel,
        };
        let input = match Input::open(&source, listen.channels, RATE) {
            Ok(input) => input,
            Err(error) => {
                report(State::WaitingForInput(error.to_string()), 0.0);
                std::thread::sleep(RETRY);
                continue;
            }
        };
        let sink = Sink::Device {
            name: listen.output.clone(),
            first_channel: 0,
        };
        let output = match Output::open(&sink, 2, RATE) {
            Ok(output) => output,
            Err(error) => {
                report(State::WaitingForOutput(error.to_string()), 0.0);
                std::thread::sleep(RETRY);
                continue;
            }
        };
        play(&listen, input, &output);
    }
}

/// Moves audio from `input` to `output` until the input stalls or either
/// device goes.
fn play(listen: &Listen, mut input: Input, output: &Output) {
    let gain = 10f32.powf(listen.gain_db / 20.0);
    let channels = usize::from(listen.channels);
    let mut taken = vec![0.0; CHUNK * channels];
    let mut played = vec![0.0; CHUNK * 2];
    let started = Instant::now();
    let mut moved = 0u64;
    let mut looked = Instant::now();
    let mut underruns = input.counts().1;
    loop {
        let due = (started.elapsed().as_secs_f64() * f64::from(RATE)) as u64;
        while moved + CHUNK as u64 <= due {
            input.read(&mut taken);
            for (frame, out) in taken
                .chunks_exact(channels)
                .zip(played.as_chunks_mut::<2>().0)
            {
                let left = frame[0] * gain;
                let right = frame.get(1).map_or(left, |right| right * gain);
                out[0] = left;
                out[1] = right;
            }
            output.write(&played, None);
            moved += CHUNK as u64;
        }
        if looked.elapsed() >= REPORT {
            looked = Instant::now();
            let now = input.counts().1;
            let stalled = now.saturating_sub(underruns) > STALLED;
            underruns = now;
            let (inputs, outputs) = triib_stream::audio::device_names();
            let gone = |name: &Option<String>, names: &[String]| {
                name.as_ref().is_some_and(|name| !names.contains(name))
            };
            if stalled || gone(&listen.input, &inputs) {
                report(State::WaitingForInput("stopped".into()), 0.0);
                return;
            }
            if gone(&listen.output, &outputs) {
                report(State::WaitingForOutput("gone".into()), 0.0);
                return;
            }
            report(State::Playing, output.take_peak());
        }
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// Tells the app, one JSON line each time.
fn report(state: State, peak: f32) {
    let status = Status { state, peak };
    if let Ok(line) = serde_json::to_string(&status) {
        let mut out = std::io::stdout().lock();
        let _ = writeln!(out, "{line}");
        let _ = out.flush();
    }
}

/// Runs the helper for `listen` and keeps it running, passing on what it
/// says: started again when it ends, and killed and started again when it
/// stops reporting. Dropped, it stops the helper.
pub struct Supervisor {
    stop: Arc<AtomicBool>,
    child: Arc<Mutex<Option<Child>>>,
}

impl Supervisor {
    pub fn start(listen: Listen, report: impl Fn(Status) + Send + 'static) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let child: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
        let (stopping, running) = (stop.clone(), child.clone());
        std::thread::spawn(move || {
            let Ok(settings) = serde_json::to_string(&listen) else {
                return;
            };
            while !stopping.load(Ordering::Relaxed) {
                report(Status {
                    state: State::Starting,
                    peak: 0.0,
                });
                let spawned = std::env::current_exe().and_then(|program| {
                    Command::new(program)
                        .arg(FLAG)
                        .arg(&settings)
                        .stdin(Stdio::null())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::null())
                        .spawn()
                });
                let Ok(mut spawned) = spawned else {
                    std::thread::sleep(RESTART);
                    continue;
                };
                let lines = spawned.stdout.take().map(BufReader::new);
                *running.lock().unwrap_or_else(PoisonError::into_inner) = Some(spawned);
                let (sender, said) = mpsc::channel();
                if let Some(lines) = lines {
                    std::thread::spawn(move || {
                        for line in lines.lines().map_while(Result::ok) {
                            if sender.send(line).is_err() {
                                return;
                            }
                        }
                    });
                }
                // Until it ends, says nothing for too long, or is stopped.
                let mut heard = Instant::now();
                while !stopping.load(Ordering::Relaxed) && heard.elapsed() < HUNG {
                    match said.recv_timeout(Duration::from_millis(500)) {
                        Ok(line) => {
                            heard = Instant::now();
                            if let Ok(status) = serde_json::from_str::<Status>(&line) {
                                report(status);
                            }
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                }
                if let Some(mut ended) = running
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .take()
                {
                    let _ = ended.kill();
                    let _ = ended.wait();
                }
                if !stopping.load(Ordering::Relaxed) {
                    std::thread::sleep(RESTART);
                }
            }
        });
        Self { stop, child }
    }
}

impl Drop for Supervisor {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(mut child) = self
            .child
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
        {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Settings out of bounds are brought within them, and what the
    /// helper says reads back as it was said.
    #[test]
    fn settings_and_reports_keep_their_shape() {
        let listen = Listen {
            channels: 5,
            gain_db: 99.0,
            ..Listen::default()
        }
        .bounded();
        assert_eq!((listen.channels, listen.gain_db), (2, 24.0));
        let status = Status {
            state: State::WaitingForOutput("gone".into()),
            peak: 0.5,
        };
        let line = serde_json::to_string(&status).unwrap();
        assert_eq!(
            line,
            r#"{"state":"waiting_for_output","detail":"gone","peak":0.5}"#
        );
        assert_eq!(serde_json::from_str::<Status>(&line).unwrap(), status);
    }
}
