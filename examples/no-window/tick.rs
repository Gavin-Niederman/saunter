use std::time::Instant;

use saunter::derive::Interpolate;
use saunter::snapshot::Snapshot;

#[derive(Debug, Clone, Interpolate)]
pub struct NoWindowTick {
    pub time: Instant,
    pub val: f32,
}
impl Snapshot for NoWindowTick {
    fn get_time(&self) -> &Instant {
        &self.time
    }
}
impl NoWindowTick {
    pub fn new(time: Instant, val: f32) -> Self {
        Self { time, val }
    }
}
