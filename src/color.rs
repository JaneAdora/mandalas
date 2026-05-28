//! Hue-shift an RGB color via an HSL round-trip.

pub type Rgb = (u8, u8, u8);

/// Shift the hue of `rgb` by `deg` degrees, keeping saturation and lightness.
pub fn shift_hue(rgb: Rgb, deg: f64) -> Rgb {
    let (r, g, b) = (rgb.0 as f64 / 255.0, rgb.1 as f64 / 255.0, rgb.2 as f64 / 255.0);
    let mx = r.max(g).max(b);
    let mn = r.min(g).min(b);
    let l = (mx + mn) / 2.0;
    let (mut h, s) = if (mx - mn).abs() < f64::EPSILON {
        (0.0, 0.0)
    } else {
        let d = mx - mn;
        let s = if l > 0.5 { d / (2.0 - mx - mn) } else { d / (mx + mn) };
        let h = if (mx - r).abs() < f64::EPSILON {
            ((g - b) / d) + if g < b { 6.0 } else { 0.0 }
        } else if (mx - g).abs() < f64::EPSILON {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };
        (h / 6.0, s)
    };
    h = (h * 360.0 + deg).rem_euclid(360.0) / 360.0;

    fn h2r(p: f64, q: f64, mut t: f64) -> f64 {
        if t < 0.0 { t += 1.0; }
        if t > 1.0 { t -= 1.0; }
        if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
        if t < 1.0 / 2.0 { return q; }
        if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
        p
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let rr = (h2r(p, q, h + 1.0 / 3.0) * 255.0).round() as u8;
    let gg = (h2r(p, q, h) * 255.0).round() as u8;
    let bb = (h2r(p, q, h - 1.0 / 3.0) * 255.0).round() as u8;
    (rr, gg, bb)
}

/// Linear interpolation between two RGB colors. `t=0` returns `a`, `t=1` returns `b`.
pub fn lerp(a: Rgb, b: Rgb, t: f64) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    (
        (a.0 as f64 + (b.0 as f64 - a.0 as f64) * t).round() as u8,
        (a.1 as f64 + (b.1 as f64 - a.1 as f64) * t).round() as u8,
        (a.2 as f64 + (b.2 as f64 - a.2 as f64) * t).round() as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_shift_is_identity() {
        let c = (200, 100, 50);
        assert_eq!(shift_hue(c, 0.0), c);
    }

    #[test]
    fn full_360_shift_round_trips_within_one() {
        let c = (200, 100, 50);
        let shifted = shift_hue(c, 360.0);
        assert!((shifted.0 as i32 - c.0 as i32).abs() <= 1);
        assert!((shifted.1 as i32 - c.1 as i32).abs() <= 1);
        assert!((shifted.2 as i32 - c.2 as i32).abs() <= 1);
    }

    #[test]
    fn negative_shift_normalises() {
        let c = (200, 100, 50);
        assert_eq!(shift_hue(c, -120.0), shift_hue(c, 240.0));
    }

    #[test]
    fn lerp_endpoints() {
        let a = (0, 0, 0);
        let b = (200, 100, 50);
        assert_eq!(lerp(a, b, 0.0), a);
        assert_eq!(lerp(a, b, 1.0), b);
    }

    #[test]
    fn lerp_midpoint() {
        let a = (0, 0, 0);
        let b = (200, 100, 50);
        assert_eq!(lerp(a, b, 0.5), (100, 50, 25));
    }
}
