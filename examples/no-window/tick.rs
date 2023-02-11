use std::time::Instant;

use saunter::{
    math::{self, MathError},
    tick::Tick,
};

#[derive(Debug, Clone)]
pub struct NoWindowTick {
    pub time: Instant,
    pub val: f32,
}
impl Tick for NoWindowTick {
    fn lerp(a: &Self, b: &Self, t: f32) -> Result<Self, MathError> {
        Ok(Self {
            time: math::lerp_instant(&a.time, &b.time, t)?,
            val: math::lerp(a.val, b.val, t),
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
