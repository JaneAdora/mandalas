//! Sacred geometry: concentric ring ripples + nested Star-of-David counter-rotating pairs.

use crate::color::shift_hue;
use crate::controls::{Common, Params};
use crate::motion::osc;
use crate::theme::{ROSE, LAVENDER, MUSTARD, MAGENTA};
use super::{sample_circle, sample_polygon, Frame};

pub fn render(p: &Params, t: f64, c: &Common, out: &mut Frame) {
    let rings = p.get(0) as i32;
    let triangles = p.get(1) as i32;
    let scale_pct = p.get(2);

    let pulse = (c.pulse_depth / 100.0).clamp(0.0, 1.0);
    let rate = c.pulse_rate;
    let density = 0.6 / c.stroke.max(0.3);

    // Concentric circles, radially rippling
    for i in 1..=rings {
        let base_r = 12.0 + (i as f64) * 11.0;
        let ripple = osc(t, rate, -(i as f64) * 0.8) * 4.0 * pulse;
        let col = shift_hue(ROSE, c.hue + (i as f64) * 6.0);
        sample_circle(0.0, 0.0, base_r + ripple, col, density, out);
    }

    // Counter-rotating Star-of-David pairs, breathing scale
    for i in 0..triangles {
        let t_norm = if triangles <= 1 { 0.0 } else { i as f64 / (triangles - 1) as f64 };
        let base_scale = 0.95 - t_norm * (0.95 - scale_pct / 100.0);
        let breathe = 1.0 + osc(t, rate * 0.7, i as f64 * std::f64::consts::FRAC_PI_2) * 0.1 * pulse;
        let r = 90.0 * base_scale * breathe;
        let direction = if i % 2 == 0 { 1.0 } else { -1.0 };
        let speed_mult = 1.0 + (i as f64) * 0.382;
        let layer_rot = t * c.speed * 8.0 * direction * speed_mult;

        let col = if i % 2 == 0 {
            shift_hue(LAVENDER, c.hue + (i as f64) * 14.0)
        } else {
            shift_hue(MUSTARD, c.hue + (i as f64) * 14.0)
        };

        let tri_at = |rot_deg: f64| {
            let mut pts = [(0.0_f64, 0.0_f64); 3];
            for k in 0..3 {
                let a = (rot_deg + (k as f64) * 120.0).to_radians();
                pts[k] = (a.sin() * r, -a.cos() * r);
            }
            pts
        };

        let p0 = tri_at(layer_rot);
        let p60 = tri_at(layer_rot + 60.0);
        sample_polygon(&p0, col, density, out);
        sample_polygon(&p60, col, density, out);
    }

    // Center pulse dot
    let core_pulse = 1.0 + osc(t, rate * 1.1, 0.0) * 0.5 * pulse;
    let core_r = (3.2 * core_pulse).max(0.5);
    let col = shift_hue(MAGENTA, c.hue);
    sample_circle(0.0, 0.0, core_r, col, density, out);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controls::{schema, Params};
    use crate::render::Mandala;

    #[test]
    fn sacred_emits_points() {
        let s = schema(Mandala::Sacred);
        let p = Params::defaults(s);
        let c = Common::default();
        let mut f = Frame::default();
        render(&p, 0.0, &c, &mut f);
        assert!(f.points.len() > 100);
    }
}
