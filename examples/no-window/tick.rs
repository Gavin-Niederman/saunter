use std::time::Instant;

use saunter::{tick::Tick, math::{MathError, self}};

#[derive(Debug, Clone)]
pub struct NoWindowTick {
    pub time: Instant,
    pub val: f32,
}
impl Tick for NoWindowTick {
    fn lerp(&self, b: &Self, t: f32) -> Result<Self, MathError> {
        Ok( Self {
            time: math::lerp_instant(&self.time, &b.time, t)?,
            val: math::lerp(self.val, b.val, t)?,
        })
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