use std::time::Instant;

use saunter::interpolate::{linear, Interpolate};
use saunter::tick::Snapshot;

#[derive(Debug, Clone)]
pub struct NoWindowTick {
    pub time: Instant,
    pub val: f32,
}
impl Snapshot for NoWindowTick {
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
impl NoWindowTick {
    pub fn new(time: Instant, val: f32) -> Self {
        Self { time, val }
    }
}
