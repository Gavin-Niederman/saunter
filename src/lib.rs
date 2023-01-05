#![feature(associated_type_defaults)]

pub mod error;
pub mod event;
pub mod listener;
pub mod math;
pub mod tick;
pub mod tickloop;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp() {
        assert_eq!(math::lerp(0.0, 100.0, 0.75).unwrap(), 75.0);

        let vals = vec![(0.0, 1.0), (0.5, 1.0), (0.0, 2.0)];
        assert_eq!(math::lerp_vec(vals, 0.5).unwrap(), vec![0.5, 0.75, 1.0]);
    }
}
