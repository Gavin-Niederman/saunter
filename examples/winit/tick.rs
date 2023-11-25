use saunter::derive::Interpolate;
use std::time::Instant;

#[derive(Clone, Debug, Interpolate)]
pub struct WinitTick {
    time: Instant,
    pub val: f32,
}
impl saunter::snapshot::Snapshot for WinitTick {
    fn get_time(&self) -> &Instant {
        &self.time
    }
}

impl WinitTick {
    pub fn new(time: Instant, val: f32) -> Self {
        WinitTick { time, val }
    }
}
