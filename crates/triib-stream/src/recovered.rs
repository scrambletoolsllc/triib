//! A media clock recovered from a CRF stream: the gPTP times its
//! timestamps give its events, fitted to a line that a talker group's
//! samples then follow.
//!
//! The timestamps are the talker's numbers, not times of arrival, so the
//! line is exact but for their nanosecond steps: its rate comes from the
//! first and last events of the last two seconds, its phase from the
//! last.

use std::collections::VecDeque;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

/// How much of the stream the rate is taken over, in nanoseconds.
const WINDOW: i64 = 2_000_000_000;
/// How much of the stream the first fit needs.
const LEAST_SPAN: i64 = 250_000_000;
/// How far from its nominal rate the clock may run, in parts per million,
/// before its stream is taken for a broken one.
const MOST_PPM: i128 = 1000;

/// The recovered clock as a line: event `event` at gPTP time `time`, each
/// next one `period` on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fit {
    /// Counts the times the clock started again: each start numbers its
    /// events afresh.
    pub generation: u64,
    pub event: i64,
    pub time: i64,
    /// Nanoseconds from one event to the next, in 32.32 fixed point.
    pub period: u64,
    /// The clock's nominal rate, in events a second.
    pub base_frequency: u32,
}

impl Fit {
    /// The gPTP time of event `sub / per`, a fraction of an event as a
    /// talker's samples at a higher rate than the clock's fall.
    pub fn time_at(&self, sub: i128, per: u64) -> i64 {
        let since = sub - i128::from(self.event) * i128::from(per);
        self.time + ((since * i128::from(self.period) / i128::from(per)) >> 32) as i64
    }

    /// How far the clock runs from its nominal rate, in parts per
    /// million: above zero when fast.
    pub fn ppm(&self) -> f64 {
        let nominal = 1e9 / f64::from(self.base_frequency.max(1));
        let period = self.period as f64 / 4_294_967_296.0;
        (nominal / period - 1.0) * 1e6
    }

    /// The first event at or after gPTP time `time`, in `1 / per` of an
    /// event.
    pub fn sub_at(&self, time: i64, per: u64) -> i128 {
        let since = (i128::from(time) - i128::from(self.time)) << 32;
        let subs = div_ceil(since * i128::from(per), i128::from(self.period));
        i128::from(self.event) * i128::from(per) + subs
    }
}

/// `numerator / denominator` rounded up, for a positive denominator.
pub(crate) fn div_ceil(numerator: i128, denominator: i128) -> i128 {
    let quotient = numerator.div_euclid(denominator);
    if numerator.rem_euclid(denominator) == 0 {
        quotient
    } else {
        quotient + 1
    }
}

#[derive(Default)]
struct State {
    generation: u64,
    /// The events seen over the window and their times, oldest first.
    seen: VecDeque<(i64, i64)>,
    fit: Option<Fit>,
    /// The talker's media reset bit, which it toggles as its clock starts
    /// again.
    media_reset: Option<bool>,
}

impl State {
    /// Forgets the clock, to recover it afresh.
    fn restart(&mut self) {
        self.generation += 1;
        self.seen.clear();
        self.fit = None;
    }
}

/// A media clock recovered from a CRF stream, shared between the listener
/// that takes the stream and the talkers that follow it. Clones share it.
#[derive(Clone, Default)]
pub struct RecoveredClock {
    state: Arc<Mutex<State>>,
}

impl PartialEq for RecoveredClock {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.state, &other.state)
    }
}

impl fmt::Debug for RecoveredClock {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RecoveredClock")
            .field("fit", &self.fit())
            .finish()
    }
}

impl RecoveredClock {
    pub fn new() -> Self {
        Self::default()
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The clock as a line, once it is recovered.
    pub fn fit(&self) -> Option<Fit> {
        self.lock().fit
    }

    pub fn locked(&self) -> bool {
        self.lock().fit.is_some()
    }

    /// Takes a CRF frame's timestamps, each `interval` events of a clock
    /// at `base_frequency` from the last, and its media reset bit.
    pub fn take(
        &self,
        timestamps: impl IntoIterator<Item = u64>,
        interval: u16,
        base_frequency: u32,
        media_reset: bool,
    ) {
        if interval == 0 || base_frequency == 0 {
            return;
        }
        let mut state = self.lock();
        if state.media_reset.replace(media_reset) == Some(!media_reset) {
            state.restart();
        }
        let interval = i64::from(interval);
        let nominal = interval as f64 * 1e9 / f64::from(base_frequency);
        let nominal_period = (1_000_000_000i128 << 32) / i128::from(base_frequency);
        for time in timestamps {
            let time = time as i64;
            let event = match state.seen.back() {
                None => 0,
                Some(&(last_event, last_time)) => {
                    // Frames lost leave whole intervals out; anything else
                    // is a clock that jumped.
                    let gap = (time - last_time) as f64;
                    let intervals = (gap / nominal).round();
                    if !(1.0..=1000.0).contains(&intervals)
                        || (gap - intervals * nominal).abs() > nominal / 100.0
                    {
                        state.restart();
                        0
                    } else {
                        last_event + intervals as i64 * interval
                    }
                }
            };
            state.seen.push_back((event, time));
            while state.seen.len() > 2
                && state
                    .seen
                    .front()
                    .is_some_and(|&(_, first)| time - first > WINDOW)
            {
                state.seen.pop_front();
            }
            let Some(&(first_event, first_time)) = state.seen.front() else {
                continue;
            };
            if time - first_time < LEAST_SPAN || event <= first_event {
                continue;
            }
            let period = (i128::from(time - first_time) << 32) / i128::from(event - first_event);
            if (period - nominal_period).abs() * 1_000_000 > nominal_period * MOST_PPM {
                state.restart();
                continue;
            }
            state.fit = Some(Fit {
                generation: state.generation,
                event,
                time,
                period: period as u64,
                base_frequency,
            });
        }
    }

    /// The stream stopped: the clock is lost until it comes again.
    pub fn lose(&self) {
        let mut state = self.lock();
        state.restart();
        state.media_reset = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 500 frames a second of one timestamp each 96 events at 48 kHz, of
    /// a clock `ppm` fast, from `start`.
    fn feed(clock: &RecoveredClock, start: u64, frames: u64, ppm: f64) {
        let period = 2_000_000.0 / (1.0 + ppm / 1e6);
        for frame in 0..frames {
            let time = start + (frame as f64 * period) as u64;
            clock.take([time], 96, 48_000, false);
        }
    }

    /// A clock 50 ppm fast is recovered to well under a nanosecond an
    /// event, and its line meets the timestamps.
    #[test]
    fn a_fast_clock_is_recovered() {
        let clock = RecoveredClock::new();
        feed(&clock, 1_000_000_000, 100, 0.0);
        assert!(!clock.locked(), "a fifth of a second is too little");
        let clock = RecoveredClock::new();
        feed(&clock, 1_000_000_000, 1500, 50.0);
        let fit = clock.fit().expect("locked");
        let nominal = 1e9 / 48_000.0;
        let period = fit.period as f64 / 4_294_967_296.0;
        assert!((period - nominal / (1.0 + 50e-6)).abs() < 0.001, "{period}");
        assert!((fit.ppm() - 50.0).abs() < 0.01, "{}", fit.ppm());
        // The line through the last event gives the one before it, a
        // frame back.
        let before = fit.time_at(i128::from(fit.event - 96), 1);
        let expected = 1_000_000_000 + (1498.0 * 2_000_000.0 / (1.0 + 50e-6)) as i64;
        assert!((before - expected).abs() <= 1, "{before} {expected}");
        // At 192 kHz, four of a talker's samples to an event.
        assert_eq!(
            fit.time_at(i128::from(fit.event) * 4 + 4, 4),
            fit.time_at(i128::from(fit.event + 1), 1)
        );
        let sub = fit.sub_at(fit.time + 1, 4);
        assert_eq!(sub, i128::from(fit.event) * 4 + 1);
    }

    /// Lost frames leave the count of events as it was; a jump, a media
    /// reset or silence start it again.
    #[test]
    fn a_jump_starts_the_clock_again() {
        let clock = RecoveredClock::new();
        feed(&clock, 1_000_000_000, 1000, 0.0);
        let fit = clock.fit().unwrap();
        // Ten frames lost.
        clock.take([1_000_000_000 + 1010 * 2_000_000], 96, 48_000, false);
        let after = clock.fit().unwrap();
        assert_eq!(after.generation, fit.generation);
        assert_eq!(after.event, fit.event + 11 * 96);
        // Half a frame off.
        let jumped = 1_000_000_000 + 1011 * 2_000_000 + 1_000_000;
        clock.take([jumped], 96, 48_000, false);
        assert!(clock.fit().is_none());
        feed(&clock, jumped + 2_000_000, 500, 0.0);
        assert_eq!(clock.fit().unwrap().generation, fit.generation + 1);
        clock.take([6_000_000_000], 96, 48_000, true);
        assert!(clock.fit().is_none(), "the media reset bit toggled");
        clock.lose();
        assert!(!clock.locked());
    }
}
