use std::f32::consts::PI;
use std::time::Duration;
use rand::prelude::*;
use rodio::source::Source;

#[derive(Clone, Debug)]
pub struct TriangleWave {
    freq: f32,
    num_sample: usize,
}

impl TriangleWave {
    #[inline]
    pub fn new(freq: f32) -> TriangleWave {
        TriangleWave {
            freq,
            num_sample: 0,
        }
    }
}

impl Iterator for TriangleWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = 2.0 / PI
            * (2.0 * PI * self.freq * self.num_sample as f32 / self.sample_rate() as f32)
                .sin()
                .asin();
        Some(value)
    }
}

impl Source for TriangleWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct SquareWave {
    freq: f32,
    num_sample: usize,
}

impl SquareWave {
    #[inline]
    pub fn new(freq: f32) -> SquareWave {
        SquareWave {
            freq,
            num_sample: 0,
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value =
            2.0 * PI * self.freq * (self.num_sample as f32 / self.sample_rate() as f32);
        Some(value.sin().signum())
    }
}

impl Source for SquareWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct Noise {}

impl Noise {
    #[inline]
    pub fn new() -> Noise {
        Noise {}
    }
}

impl Iterator for Noise {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let value = rand::thread_rng().gen_range(-1.0..1.0);
        Some(value)
    }
}

impl Source for Noise {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}