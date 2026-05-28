//! Mandala rendering — each mandala is a free function over (params, t, common).
//! Renderers append shape sample points to a `Frame` then ratatui's Canvas
//! paints them via `ctx.draw(&Points)`.

pub mod sacred;
pub mod lotus;
pub mod spirograph;
pub mod star;
pub mod flower;
pub mod interlace;

use crate::color::Rgb;
use crate::controls::{Common, Params};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mandala {
    Sacred, Lotus, Spirograph, Star, Flower, Interlace,
}

impl Mandala {
    pub const ALL: &'static [Mandala] = &[
        Mandala::Sacred, Mandala::Lotus, Mandala::Spirograph,
        Mandala::Star, Mandala::Flower, Mandala::Interlace,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Mandala::Sacred     => "Sacred Geometry",
            Mandala::Lotus      => "Lotus",
            Mandala::Spirograph => "Spirograph",
            Mandala::Star       => "Star Lattice",
            Mandala::Flower     => "Flower of Life",
            Mandala::Interlace  => "Interlace",
        }
    }

    pub fn slug(&self) -> &'static str {
        match self {
            Mandala::Sacred     => "sacred",
            Mandala::Lotus      => "lotus",
            Mandala::Spirograph => "spirograph",
            Mandala::Star       => "star",
            Mandala::Flower     => "flower",
            Mandala::Interlace  => "interlace",
        }
    }

    pub fn from_slug(s: &str) -> Option<Mandala> {
        Mandala::ALL.iter().copied().find(|m| m.slug() == s)
    }

    pub fn next(self) -> Mandala {
        let i = Mandala::ALL.iter().position(|m| *m == self).unwrap_or(0);
        Mandala::ALL[(i + 1) % Mandala::ALL.len()]
    }

    pub fn prev(self) -> Mandala {
        let i = Mandala::ALL.iter().position(|m| *m == self).unwrap_or(0);
        Mandala::ALL[(i + Mandala::ALL.len() - 1) % Mandala::ALL.len()]
    }
}

/// One sampled point on the canvas, with color.
#[derive(Debug, Clone, Copy)]
pub struct Pt {
    pub x: f64,
    pub y: f64,
    pub color: Rgb,
}

/// A frame's worth of points, in canvas coordinates centred at (0, 0).
#[derive(Debug, Default)]
pub struct Frame {
    pub points: Vec<Pt>,
}

impl Frame {
    pub fn push(&mut self, x: f64, y: f64, color: Rgb) {
        self.points.push(Pt { x, y, color });
    }
}

/// Sample N evenly-spaced points along a circle centred at (cx, cy).
pub fn sample_circle(cx: f64, cy: f64, r: f64, color: Rgb, density: f64, out: &mut Frame) {
    if r < 0.5 { return; }
    let n = (2.0 * std::f64::consts::PI * r / density).ceil() as usize;
    let n = n.max(8);
    for i in 0..n {
        let a = i as f64 / n as f64 * crate::motion::TAU;
        out.push(cx + a.cos() * r, cy + a.sin() * r, color);
    }
}

/// Sample points along a line from (x0, y0) to (x1, y1).
pub fn sample_line(x0: f64, y0: f64, x1: f64, y1: f64, color: Rgb, density: f64, out: &mut Frame) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.5 { return; }
    let n = (len / density).ceil() as usize;
    let n = n.max(2);
    for i in 0..=n {
        let t = i as f64 / n as f64;
        out.push(x0 + dx * t, y0 + dy * t, color);
    }
}

/// Sample a closed polygon by sampling each edge.
pub fn sample_polygon(pts: &[(f64, f64)], color: Rgb, density: f64, out: &mut Frame) {
    if pts.len() < 2 { return; }
    for i in 0..pts.len() {
        let (x0, y0) = pts[i];
        let (x1, y1) = pts[(i + 1) % pts.len()];
        sample_line(x0, y0, x1, y1, color, density, out);
    }
}

pub fn render(m: Mandala, p: &Params, t: f64, c: &Common, out: &mut Frame) {
    match m {
        Mandala::Sacred     => sacred::render(p, t, c, out),
        Mandala::Lotus      => lotus::render(p, t, c, out),
        Mandala::Spirograph => spirograph::render(p, t, c, out),
        Mandala::Star       => star::render(p, t, c, out),
        Mandala::Flower     => flower::render(p, t, c, out),
        Mandala::Interlace  => interlace::render(p, t, c, out),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn name_and_slug_round_trip() {
        for m in Mandala::ALL { assert_eq!(Mandala::from_slug(m.slug()), Some(*m)); }
    }
    #[test]
    fn cycling_returns_to_start() {
        let mut m = Mandala::Sacred;
        for _ in 0..Mandala::ALL.len() { m = m.next(); }
        assert_eq!(m, Mandala::Sacred);
    }
    #[test]
    fn sample_circle_pushes_points() {
        let mut f = Frame::default();
        sample_circle(0.0, 0.0, 10.0, (255, 0, 0), 1.0, &mut f);
        assert!(f.points.len() >= 8);
        for p in &f.points {
            let r = (p.x * p.x + p.y * p.y).sqrt();
            assert!((r - 10.0).abs() < 0.001);
        }
    }
    #[test]
    fn sample_line_pushes_points() {
        let mut f = Frame::default();
        sample_line(0.0, 0.0, 10.0, 0.0, (0,0,0), 1.0, &mut f);
        assert!(f.points.len() >= 2);
        assert!(f.points.first().unwrap().x < 0.01);
        assert!((f.points.last().unwrap().x - 10.0).abs() < 0.01);
    }
}
