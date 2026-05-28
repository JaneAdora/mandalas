//! Interlace: orbiting overlapping circles with hue chasing around the ring count.

use crate::color::shift_hue;
use crate::controls::{Common, Params};
use crate::motion::osc;
use crate::theme::{LAVENDER, ROSE, MUSTARD, SAGE, MAGENTA};
use super::{sample_circle, Frame};

pub fn render(p: &Params, t: f64, c: &Common, out: &mut Frame) {
    let rings = p.get(0) as i32;
    let radius = p.get(1);
    let orbit = p.get(2);

    let pulse = (c.pulse_depth / 100.0).clamp(0.0, 1.0);
    let rate = c.pulse_rate;
    let density = 0.6 / c.stroke.max(0.3);
    let palette = [LAVENDER, ROSE, MUSTARD, SAGE, MAGENTA];

    let orbital_rot = t * c.speed * 6.0;
    let orbit_r = orbit * (1.0 + osc(t, rate * 0.5, 0.0) * 0.12 * pulse);

    for i in 0..rings {
        let a_deg = i as f64 * 360.0 / rings as f64 + orbital_rot;
        let a = a_deg.to_radians();
        let cx = orbit_r * a.cos();
        let cy = orbit_r * a.sin();
        let ring_pulse = 1.0 + osc(t, rate, (i as f64) * 0.5) * 0.1 * pulse;
        let r = radius * ring_pulse;
        let col = shift_hue(palette[i as usize % palette.len()], c.hue + (i as f64) * 28.0 + t * 18.0 * pulse);
        sample_circle(cx, cy, r, col, density / 1.4, out);
    }

    let core_pulse = 1.0 + osc(t, rate * 1.3, 0.0) * 0.3 * pulse;
    sample_circle(0.0, 0.0, (3.5 * core_pulse).max(0.5), shift_hue(MAGENTA, c.hue), density, out);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controls::{schema, Params};
    use crate::render::Mandala;
    #[test]
    fn interlace_emits_points() {
        let p = Params::defaults(schema(Mandala::Interlace));
        let mut f = Frame::default();
        render(&p, 0.0, &Common::default(), &mut f);
        assert!(f.points.len() > 100);
    }
}
