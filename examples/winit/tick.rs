use saunter::interpolate::{linear, Interpolate};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct WinitTick {
    time: Instant,
    pub val: f32,
}
impl saunter::tick::Snapshot for WinitTick {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Self {
            time: Instant::interpolate(&a.time, &b.time, t, linear),
            val: f32::interpolate(&a.val, &b.val, t, linear),
        }
    }

    fn get_time(&self) -> &Instant {
        &self.time
    }
}

impl WinitTick {
    pub fn new(time: Instant, val: f32) -> Self {
        WinitTick { time, val }
    }
}
