use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::math::MathError;
use crate::tick::TickError;

#[derive(Debug)]
pub enum SaunterError {
    TickError(TickError),
    MathError(MathError),
}
impl Error for SaunterError {}
impl Display for SaunterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SaunterError::TickError(err) => err.fmt(f),
            SaunterError::MathError(err) => err.fmt(f),
        }
    }
}
