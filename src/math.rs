use std::{
    error::Error,
    fmt::{self, Display, Formatter},
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

#[derive(Debug)]
pub enum MathError {
    TOutOfRange,
}
impl Error for MathError {}

impl Display for MathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MathError::TOutOfRange => write!(f, "t must be between 0 and 1"),
        }
    }
}
