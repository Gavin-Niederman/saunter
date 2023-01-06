//! Contains the [`Tick`](crate::tick::Tick) trait and [`Ticks`](crate::tick::Ticks) struct.

use crate::{error::SaunterError, math::MathError};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    time::Instant,
};

// A snapshot of the state of the game engine, or current scene at the time of creation.
pub trait Tick: Clone + Debug {
    /// Lerps between self and b by t. t should be between 0 and 1.
    fn lerp(&self, b: &Self, t: f32) -> Result<Self, MathError>;
    /// Returns the time that the tick was created.
    fn get_time(&self) -> &Instant;
}

/// A struct that holds the last tick and the most recent tick. It is used to lerp between the two ticks.
pub struct Ticks<T: Tick> {
    pub last_tick: Option<T>,
    pub new_tick: T,
}

impl<T: Tick> Ticks<T> {
    /// Creates a new Ticks struct with the provided first tick.
    /// The first tick should be the default state of the game engine or scene.
    pub fn new(first: T) -> Self {
        Ticks {
            last_tick: None,
            new_tick: first,
        }
    }

    /// Lerps between the last tick and the new tick by t. t should be between 0 and 1.
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

    /// Drops last tick and replaces it with new tick, and then replaces new tick with the new new tick.
    pub fn update(&mut self, new_tick: T) {
        self.last_tick = Some(self.new_tick.clone());
        self.new_tick = new_tick;
    }
}

/// Contains errors that can occur when using ticks.
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
