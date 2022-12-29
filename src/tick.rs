use crate::{error::SaunterError, math::MathError};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

pub trait Tick: Clone {
    fn lerp(&self, b: &Self, t: f32) -> Result<Self, MathError>;
}

pub struct Ticks<T: Tick> {
    pub last_tick: Option<T>,
    pub new_tick: T,
}

impl<T: Tick> Ticks<T> {
    pub fn new(first: T) -> Self {
        Ticks {
            last_tick: None,
            new_tick: first,
        }
    }

    pub fn lerp(&self, t: f32) -> Result<T, SaunterError> {
        if let Some(last) = &self.last_tick {
            match self.new_tick.lerp(last, t) {
                Ok(val) => Ok(val),
                Err(err) => Err(SaunterError::MathError(err)),
            }
        } else {
            Err(SaunterError::TickError(TickError::TooFewTicks))
        }
    }

    pub fn update(&mut self, new_tick: T) {
        self.last_tick = Some(self.new_tick.clone());
        self.new_tick = new_tick;
    }
}

#[derive(Debug)]
pub enum TickError {
    TooFewTicks,
}
impl Error for TickError {}

impl Display for TickError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TickError::TooFewTicks => write!(f, "Not enough ticks to lerp"),
        }
    }
}
