//! Holds the error enum, which holds either a [`MathError`](crate::math::MathError) or a [`TickError`](crate::tick::TickError).

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::tick::TickError;

/// The error enum, which holds either a [`MathError`](crate::math::MathError) or a [`TickError`](crate::tick::TickError).
#[derive(Debug)]
pub enum SaunterError {
    TickError(TickError),
}
impl Error for SaunterError {}
impl Display for SaunterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SaunterError::TickError(err) => err.fmt(f),
        }
    }
}
