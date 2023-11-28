//! Contains the error type for Saunter.

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Contains errors that can occur when using Saunter.
#[derive(Debug)]
pub enum SaunterError {
    TooFewSnapshots,
    CouldNotCreateSnapshot,
}
impl Error for SaunterError {}

impl Display for SaunterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SaunterError::TooFewSnapshots => write!(f, "Not enough ticks to lerp"),
            SaunterError::CouldNotCreateSnapshot => write!(f, "Could not create a tick"),
        }
    }
}
