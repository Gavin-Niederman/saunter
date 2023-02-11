use saunter::math::{self, MathError};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct WinitTick {
    time: Instant,
    pub val: f32,
}
impl saunter::tick::Tick for WinitTick {
    fn lerp(a: &Self, b: &Self, t: f32) -> Result<Self, MathError> {
        Ok(WinitTick {
            time: math::lerp_instant(&a.time, &b.time, t)?,
            val: math::lerp(a.val, b.val, t),
        })
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
