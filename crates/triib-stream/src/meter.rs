//! The loudest samples of a stream's first two channels, which triib's
//! meters show.

use std::sync::atomic::{AtomicU32, Ordering};

/// The loudest sample in each of a stream's first two channels since they
/// were last taken, from 0 to 1: raised by the thread moving its audio,
/// taken by whoever shows them.
#[derive(Default)]
pub(crate) struct Peaks([AtomicU32; 2]);

impl Peaks {
    /// Raises each to the loudest of its channel in `samples`, frames of
    /// `channels` interleaved; with one channel, the second stays as it is.
    pub(crate) fn raise(&self, samples: &[f32], channels: usize) {
        let mut loudest = [0.0f32; 2];
        for frame in samples.chunks_exact(channels.max(1)) {
            for (most, sample) in loudest.iter_mut().zip(frame) {
                *most = most.max(sample.abs());
            }
        }
        // A float at or above zero orders as its bits do.
        for (peak, most) in self.0.iter().zip(loudest) {
            peak.fetch_max(most.to_bits(), Ordering::Relaxed);
        }
    }

    /// Each channel's loudest sample, starting both again from silence.
    pub(crate) fn take(&self) -> [f32; 2] {
        self.0
            .each_ref()
            .map(|peak| f32::from_bits(peak.swap(0, Ordering::Relaxed)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_of_the_first_two_channels_keeps_its_loudest() {
        let peaks = Peaks::default();
        // Three channels: the third is not metered.
        peaks.raise(&[0.25, -0.5, 1.0, -0.75, 0.125, 1.0], 3);
        peaks.raise(&[0.5, 0.0, 0.0], 3);
        assert_eq!(peaks.take(), [0.75, 0.5]);
        assert_eq!(peaks.take(), [0.0, 0.0]);
    }

    #[test]
    fn one_channel_raises_the_first_alone() {
        let peaks = Peaks::default();
        peaks.raise(&[0.25, -0.5], 1);
        assert_eq!(peaks.take(), [0.5, 0.0]);
    }
}
