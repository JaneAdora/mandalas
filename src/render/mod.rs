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
}
