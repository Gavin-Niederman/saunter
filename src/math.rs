use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    time::{Duration, Instant},
};

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> Result<f32, MathError> {
    if t < 0.0 || t > 1.0 {
        return Err(MathError::TOutOfRange);
    }

    Ok(a * (1.0 - t) + b * t)
}

#[inline]
pub fn lerp_vec(list: Vec<(f32, f32)>, t: f32) -> Result<Vec<f32>, MathError> {
    let mut result = Vec::new();

    for (a, b) in list {
        result.push(lerp(a, b, t)?);
    }

    Ok(result)
}

#[inline]
pub fn lerp_instant(a: &Instant, b: &Instant, t: f32) -> Result<Instant, MathError> {
    let b = b.duration_since(a.clone());
    a.checked_add(Duration::from_secs_f32(b.as_secs_f32() * t))
        .ok_or(MathError::InstantOutOfRange)
}

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

pub fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

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
