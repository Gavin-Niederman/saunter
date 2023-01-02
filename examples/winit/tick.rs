use saunter::math::{self, MathError};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct PrintTick {
    time: Instant,
    pub val: f32,
}
impl saunter::tick::Tick for PrintTick {
    fn lerp(&self, b: &Self, t: f32) -> Result<Self, MathError> {
        Ok(PrintTick {
            time: math::lerp_instant(&self.time, &b.time, t)?,
            val: math::lerp(self.val, b.val, t)?,
        })
    }

    fn get_time(&self) -> &Instant {
        &self.time
    }
}

impl PrintTick {
    pub fn new(time: Instant, val: f32) -> Self {
        PrintTick { time, val }
    }
}
