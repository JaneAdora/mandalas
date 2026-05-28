//! Flower of Life: hex-grid circles with wave propagating outward from center.

use crate::color::{shift_hue, lerp};
use crate::controls::{Common, Params};
use crate::motion::{osc, osc01};
use crate::theme::{ROSE, LAVENDER, MUSTARD, SAGE, MAGENTA, BG};
use super::{sample_circle, Frame};

pub fn render(p: &Params, t: f64, c: &Common, out: &mut Frame) {
    let layers = p.get(0) as i32;
    let radius = p.get(1);

    let pulse = (c.pulse_depth / 100.0).clamp(0.0, 1.0);
    let rate = c.pulse_rate;
    let density = 0.6 / c.stroke.max(0.3);

    let palette = [ROSE, LAVENDER, MUSTARD, SAGE];
    let sqrt3_2 = (3.0_f64).sqrt() / 2.0;

    let mut seen: Vec<(i32, i32)> = Vec::new();
    for l in 0..layers {
        for i in -l..=l {
            for j in -l..=l {
                let k = -i - j;
                if k.abs() > l { continue; }
                let ring = i.abs().max(j.abs()).max(k.abs());
                if ring != l { continue; }
                if seen.contains(&(i, j)) { continue; }
                seen.push((i, j));

                let x = radius * (i as f64 + j as f64 * 0.5);
                let y = radius * (j as f64) * sqrt3_2;
                let dist = (x * x + y * y).sqrt();
                let phase = (dist / radius) * 0.9;
                let wave = osc01(t, rate * 0.6, -phase * crate::motion::TAU);
                let r = radius * (1.0 + (wave - 0.5) * 0.14 * pulse);

                let opacity = 0.35 + 0.65 * (1.0 - pulse * 0.7 * (1.0 - wave));
                let base = palette[l as usize % palette.len()];
                let hued = shift_hue(base, c.hue + (l as f64) * 18.0 + dist * 0.6);
                let col = lerp(BG, hued, opacity);

                sample_circle(x, y, r, col, density, out);
            }
        }
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
    fn flower_emits_points() {
        let p = Params::defaults(schema(Mandala::Flower));
        let mut f = Frame::default();
        render(&p, 0.0, &Common::default(), &mut f);
        assert!(f.points.len() > 100);
    }
}
