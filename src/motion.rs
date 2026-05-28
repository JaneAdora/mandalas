//! Animation primitives shared by all renderers.

pub const TAU: f64 = std::f64::consts::TAU;

/// Sine wave in `[-1, 1]`. `hz` cycles per second, `phase` in radians.
pub fn osc(t: f64, hz: f64, phase: f64) -> f64 {
    (t * hz * TAU + phase).sin()
}

/// Sine wave in `[0, 1]`. Phase-shifted cosine of `osc`.
pub fn osc01(t: f64, hz: f64, phase: f64) -> f64 {
    0.5 + 0.5 * osc(t, hz, phase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn osc_at_zero_zero_phase_is_zero() {
        assert!(osc(0.0, 1.0, 0.0).abs() < 1e-12);
    }

    #[test]
    fn osc_quarter_period_is_one() {
        assert!((osc(0.25, 1.0, 0.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn osc01_is_in_zero_one() {
        for i in 0..100 {
            let t = i as f64 * 0.01;
            let v = osc01(t, 1.0, 0.0);
            assert!(v >= 0.0 && v <= 1.0);
        }
    }

    #[test]
    fn osc01_at_zero_is_half() {
        assert!((osc01(0.0, 1.0, 0.0) - 0.5).abs() < 1e-12);
    }
}
