use std::time::Instant;

pub trait Interpolate {
    fn interpolate(start: &Self, end: &Self, t: f32, interpolation: impl Fn(f32) -> f32) -> Self
    where
        Self: Sized;

    fn interpolate_self(&mut self, end: &Self, t: f32, interpolation: impl Fn(f32) -> f32)
    where
        Self: Sized,
    {
        *self = Self::interpolate(&self, end, t, interpolation)
    }
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

macro_rules! impl_interpolate {
    ($($t:ty),*) => {
        $(
            impl Interpolate for $t {
                fn interpolate(start: &Self, end: &Self, t: f32, interpolation: impl Fn(f32) -> f32) -> Self {
                    let start = *start as f32;
                    let end = *end as f32;
                    let t = interpolation(t);
                    lerp(start, end, t) as $t
                }
            }
        )*
    };
}

impl_interpolate!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl Interpolate for Instant {
    fn interpolate(start: &Self, end: &Self, t: f32, interpolation: impl Fn(f32) -> f32) -> Self
    where
        Self: Sized,
    {
        let start = start.elapsed().as_secs_f32();
        let end = end.elapsed().as_secs_f32();
        let t = interpolation(t);
        let secs = lerp(start, end, t);
        Instant::now() - std::time::Duration::from_secs_f32(secs)
    }
}

impl<I: Interpolate> Interpolate for Vec<I> {
    fn interpolate(start: &Self, end: &Self, t: f32, interpolation: impl Fn(f32) -> f32) -> Self
        where
            Self: Sized {
        start.iter().zip(end.iter()).map(|(a, b)| I::interpolate(a, b, t, &interpolation)).collect()
    }
}

// Some common interpolation functions

pub fn linear(t: f32) -> f32 {
    t
}

pub fn ease_in_sine(t: f32) -> f32 {
    1.0 - ((t * std::f32::consts::PI) / 2.0).cos()
}
pub fn ease_out_sine(t: f32) -> f32 {
    ((t * std::f32::consts::PI) / 2.0).sin()
}
pub fn ease_in_out_sine(t: f32) -> f32 {
    (t * std::f32::consts::PI).sin()
}

pub fn ease_in_quad(t: f32) -> f32 {
    t * t
}
pub fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}
pub fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0) / 2.0
    }
}

pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}
pub fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t)
}
pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) / 2.0
    }
}

pub fn ease_in_quart(t: f32) -> f32 {
    t * t * t * t
}
pub fn ease_out_quart(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t)
}
pub fn ease_in_out_quart(t: f32) -> f32 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) / 2.0
    }
}

pub fn ease_in_quint(t: f32) -> f32 {
    t * t * t * t * t
}
pub fn ease_out_quint(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t)
}
pub fn ease_in_out_quint(t: f32) -> f32 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0)
            * (-2.0 * t + 2.0)
            * (-2.0 * t + 2.0)
            * (-2.0 * t + 2.0)
            * (-2.0 * t + 2.0)
            / 2.0
    }
}

pub fn ease_in_expo(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else {
        2.0f32.powf(10.0 * t - 10.0)
    }
}
pub fn ease_out_expo(t: f32) -> f32 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - 2.0f32.powf(-10.0 * t)
    }
}
pub fn ease_in_out_expo(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        2.0f32.powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - 2.0f32.powf(-20.0 * t + 10.0)) / 2.0
    }
}

pub fn ease_in_circ(t: f32) -> f32 {
    1.0 - (1.0 - (t * t)).sqrt()
}
pub fn ease_out_circ(t: f32) -> f32 {
    (1.0 - (t - 1.0) * (t - 1.0)).sqrt()
}
pub fn ease_in_out_circ(t: f32) -> f32 {
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t) * (2.0 * t)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0)).sqrt() + 1.0) / 2.0
    }
}

pub fn ease_in_back(t: f32) -> f32 {
    2.70158 * t * t * t - 1.70158 * t * t
}
pub fn ease_out_back(t: f32) -> f32 {
    1.0 + 2.70158 * (t - 1.0) * (t - 1.0) * (t - 1.0) + 1.70158 * (t - 1.0) * (t - 1.0)
}
pub fn ease_in_out_back(t: f32) -> f32 {
    let c1 = 1.70158;
    let c2 = c1 * 1.525;

    if t < 0.5 {
        (2.0 * t) * (2.0 * t) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) / 2.0
    } else {
        ((2.0 * t) * (2.0 * t) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
    }
}
