//! Provides some basic math functions, such as linear interpolation. As well as the [`MathError`](crate::math::MathError) enum.

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    time::{Duration, Instant},
};

/// Linearly interpolates between two floats
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> Result<f32, MathError> {
    if t < 0.0 || t > 1.0 {
        return Err(MathError::TOutOfRange);
    }

    Ok(a * (1.0 - t) + b * t)
}

/// Linearly interpolates between a vector of (f32, f32)
#[inline]
pub fn lerp_vec(list: Vec<(f32, f32)>, t: f32) -> Result<Vec<f32>, MathError> {
    let mut result = Vec::new();

    for (a, b) in list {
        result.push(lerp(a, b, t)?);
    }

    Ok(result)
}

/// Linearly interpolates between two instants
#[inline]
pub fn lerp_instant(a: &Instant, b: &Instant, t: f32) -> Result<Instant, MathError> {
    let b = b.duration_since(a.clone());
    a.checked_add(Duration::from_secs_f32(b.as_secs_f32() * t))
        .ok_or(MathError::InstantOutOfRange)
}

/// Clamps a float between a min and max
#[inline]
pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

/// Returns the max of two floats
#[inline]
pub fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Returns the min of two floats
#[inline]
pub fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

/// The error enum for math functions
#[derive(Debug)]
pub enum MathError {
    TOutOfRange,
    InstantOutOfRange,
}
impl Error for MathError {}

impl Display for MathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MathError::TOutOfRange => write!(f, "t must be between 0 and 1"),
            MathError::InstantOutOfRange => write!(f, "Instant is out of range"),
        }
    }
}
