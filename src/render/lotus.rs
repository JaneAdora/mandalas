//! Lotus: concentric layers of petals blooming in cyclic waves.
//!
//! Each layer's petals are evenly spaced around the origin. Per petal we draw
//! an ellipse whose centre sits at distance `|cy|` from origin along the
//! petal's rotation direction. The `petal_ellipse` helper bakes the cy
//! translation in LOCAL coordinates BEFORE rotating into world space, so a
//! single call places one rotated, offset ellipse.

use crate::color::shift_hue;
use crate::controls::{Common, Params};
use crate::motion::{osc, osc01, TAU};
use crate::theme::{LAVENDER, ROSE, MUSTARD, SAGE, MAGENTA};
use super::{sample_circle, Frame};

/// Draw an ellipse with semi-axes (rx, ry), offset by `cy` along its local
/// Y axis, then rotated by `rot_deg` around the origin.
fn petal_ellipse(rx: f64, ry: f64, cy: f64, rot_deg: f64, color: (u8, u8, u8), density: f64, out: &mut Frame) {
    if rx < 0.3 || ry < 0.3 { return; }
    // Ramanujan approximation for ellipse circumference
    let circumference = std::f64::consts::PI * (3.0 * (rx + ry) - ((3.0 * rx + ry) * (rx + 3.0 * ry)).sqrt());
    let n = ((circumference / density).ceil() as usize).max(20);
    let rot = rot_deg.to_radians();
    let (cs, sn) = (rot.cos(), rot.sin());
    for i in 0..n {
        let a = i as f64 / n as f64 * TAU;
        let lx = a.cos() * rx;
        let ly = a.sin() * ry + cy;
        let x = cs * lx - sn * ly;
        let y = sn * lx + cs * ly;
        out.push(x, y, color);
    }
}

pub fn render(p: &Params, t: f64, c: &Common, out: &mut Frame) {
    let petals = p.get(0) as i32;
    let layers = p.get(1) as i32;
    let length = p.get(2);
    let core = p.get(3);

    let pulse = (c.pulse_depth / 100.0).clamp(0.0, 1.0);
    let rate = c.pulse_rate;
    let density = 0.6 / c.stroke.max(0.3);

    for layer in 0..layers {
        let ratio = 1.0 - (layer as f64) * 0.28;
        let bloom = osc01(t, rate * 0.45, -(layer as f64) * std::f64::consts::FRAC_PI_2);
        let len_mult = 1.0 - pulse * 0.45 * (1.0 - bloom);
        let width_mult = 0.8 + 0.2 * bloom;
        let ry = length * ratio * len_mult;
        let rx = 14.0 * ratio * width_mult;
        let cy = -ry * 0.55;
        let offset_deg = (layer % 2) as f64 * (180.0 / petals as f64);
        let direction = if layer % 2 == 0 { 1.0 } else { -1.0 };
        let layer_rot = t * c.speed * 4.0 * direction;
        let palette = [LAVENDER, ROSE, MUSTARD, SAGE];
        let base = palette[layer as usize % palette.len()];
        let col = shift_hue(base, c.hue + (layer as f64) * 22.0);

        for i in 0..petals {
            let a_deg = i as f64 * 360.0 / petals as f64 + offset_deg + layer_rot;
            petal_ellipse(rx, ry, cy, a_deg, col, density, out);
        }
    }

    // Core ring + dot
    let core_r = core * (1.0 + osc(t, rate * 1.3, 0.0) * 0.15 * pulse);
    let col = shift_hue(MAGENTA, c.hue);
    sample_circle(0.0, 0.0, core_r, col, density, out);
    sample_circle(0.0, 0.0, 3.0, col, density, out);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controls::{schema, Params};
    use crate::render::Mandala;

    #[test]
    fn lotus_emits_points() {
        let p = Params::defaults(schema(Mandala::Lotus));
        let mut f = Frame::default();
        render(&p, 0.0, &Common::default(), &mut f);
        assert!(f.points.len() > 100);
    }

    #[test]
    fn lotus_petals_are_offset_from_origin() {
        // With cy = -ry*0.55 != 0, the petals should NOT all be at origin —
        // their centroids should sit at distance ≈ |cy| from origin.
        let p = Params::defaults(schema(Mandala::Lotus));
        let mut f = Frame::default();
        render(&p, 0.0, &Common::default(), &mut f);
        // Find the max distance from origin — if petals are placed correctly
        // it should be roughly ry + |cy| (a petal tip), which is comfortably > 30.
        let max_d = f.points.iter()
            .map(|p| (p.x * p.x + p.y * p.y).sqrt())
            .fold(0.0_f64, f64::max);
        assert!(max_d > 30.0, "petals should extend far from origin, got max_d={}", max_d);
    }
}
