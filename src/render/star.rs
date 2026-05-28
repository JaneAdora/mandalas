//! Star Lattice: nested N-pointed stars, golden-ratio counter-rotation.

use crate::color::shift_hue;
use crate::controls::{Common, Params};
use crate::motion::osc;
use crate::theme::{LAVENDER, ROSE, MUSTARD, SAGE, MAGENTA};
use super::{sample_polygon, sample_circle, Frame};

fn star_points(n_points: i32, outer: f64, inner: f64, rot_deg: f64) -> Vec<(f64, f64)> {
    let mut pts = Vec::with_capacity((n_points * 2) as usize);
    for i in 0..(n_points * 2) {
        let r = if i % 2 == 0 { outer } else { inner };
        let a_deg = (i as f64) * 180.0 / (n_points as f64) + rot_deg;
        let a = a_deg.to_radians();
        pts.push((a.sin() * r, -a.cos() * r));
    }
    pts
}

pub fn render(p: &Params, t: f64, c: &Common, out: &mut Frame) {
    let points = p.get(0) as i32;
    let nested = p.get(1) as i32;
    let inner_pct = p.get(2);
    let phase_deg = p.get(3);

    let pulse = (c.pulse_depth / 100.0).clamp(0.0, 1.0);
    let rate = c.pulse_rate;
    let density = 0.6 / c.stroke.max(0.3);
    let palette = [LAVENDER, ROSE, MUSTARD, SAGE, MAGENTA];

    for i in 0..nested {
        let scale = 1.0 - (i as f64) * (0.7 / nested.max(1) as f64);
        let outer = 90.0 * scale;
        let inner_breathe = osc(t, rate * 0.5, (i as f64) * std::f64::consts::PI / 3.0) * 10.0 * pulse;
        let inner = outer * ((inner_pct + inner_breathe) / 100.0);
        let base_rot = (i as f64) * phase_deg;
        let direction = if i % 2 == 0 { 1.0 } else { -1.0 };
        let speed_mult = 1.0 + (i as f64) * 0.382;
        let anim_rot = t * c.speed * 10.0 * direction * speed_mult;
        let rot = base_rot + anim_rot;
        let col = shift_hue(palette[i as usize % palette.len()], c.hue + (i as f64) * 14.0);
        let pts = star_points(points, outer, inner.max(2.0), rot);
        sample_polygon(&pts, col, density, out);
    }

    let core_pulse = 1.0 + osc(t, rate * 1.4, 0.0) * 0.4 * pulse;
    sample_circle(0.0, 0.0, (3.0 * core_pulse).max(0.5), shift_hue(MAGENTA, c.hue), density, out);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controls::{schema, Params};
    use crate::render::Mandala;
    #[test]
    fn star_emits_points() {
        let p = Params::defaults(schema(Mandala::Star));
        let mut f = Frame::default();
        render(&p, 0.0, &Common::default(), &mut f);
        assert!(f.points.len() > 100);
    }
}
