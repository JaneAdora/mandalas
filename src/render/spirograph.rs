//! Spirograph: hypotrochoid curve with sin-drifted parameters and two echo trails.

use crate::color::{lerp, shift_hue};
use crate::controls::{Common, Params};
use crate::motion::{osc, TAU};
use crate::theme::{MUSTARD, MAGENTA, BG};
use super::{sample_line, sample_circle, Frame};

fn hypotrochoid_points(big_r: f64, little_r: f64, d: f64, revs: f64) -> Vec<(f64, f64)> {
    let steps = ((revs * 80.0).max(180.0)) as usize;
    let little_r = little_r.max(2.0);
    let mut pts = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let u = i as f64 / steps as f64 * revs * TAU;
        let x = (big_r - little_r) * u.cos() + d * ((big_r - little_r) / little_r * u).cos();
        let y = (big_r - little_r) * u.sin() - d * ((big_r - little_r) / little_r * u).sin();
        pts.push((x, y));
    }
    pts
}

fn draw_path(pts: &[(f64, f64)], color: (u8,u8,u8), density: f64, out: &mut Frame) {
    for w in pts.windows(2) {
        sample_line(w[0].0, w[0].1, w[1].0, w[1].1, color, density, out);
    }
}

pub fn render(p: &Params, t: f64, c: &Common, out: &mut Frame) {
    let r0 = p.get(0);
    let r1 = p.get(1);
    let d0 = p.get(2);
    let revs = p.get(3);

    let pulse = (c.pulse_depth / 100.0).clamp(0.0, 1.0);
    let rate = c.pulse_rate;
    let density = 0.7 / c.stroke.max(0.3);

    let at = |tt: f64| {
        let big_r = r0 + osc(tt, rate * 0.15, 0.0) * 4.0 * pulse;
        let little_r = (r1 + osc(tt, rate * 0.13, 1.0) * 2.0 * pulse).max(2.0);
        let d_off = d0 + osc(tt, rate * 0.11, 2.0) * 4.0 * pulse;
        hypotrochoid_points(big_r, little_r, d_off, revs)
    };

    let base = shift_hue(MUSTARD, c.hue);

    if pulse > 0.01 {
        let ghost2 = at(t - 0.7);
        let g2_col = lerp(BG, base, 0.18 * pulse + 0.1);
        draw_path(&ghost2, g2_col, density * 1.4, out);

        let ghost1 = at(t - 0.35);
        let g1_col = lerp(BG, base, 0.4 * pulse + 0.2);
        draw_path(&ghost1, g1_col, density * 1.2, out);
    }

    let now_path = at(t);
    draw_path(&now_path, base, density, out);

    let core_pulse = 1.0 + osc(t, rate * 1.5, 0.0) * 0.35 * pulse;
    let core_r = (4.0 * core_pulse).max(1.0);
    let mc = shift_hue(MAGENTA, c.hue);
    sample_circle(0.0, 0.0, core_r, mc, density, out);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controls::{schema, Params};
    use crate::render::Mandala;
    #[test]
    fn spirograph_emits_points() {
        let p = Params::defaults(schema(Mandala::Spirograph));
        let mut f = Frame::default();
        render(&p, 0.0, &Common::default(), &mut f);
        assert!(f.points.len() > 200);
    }
}
