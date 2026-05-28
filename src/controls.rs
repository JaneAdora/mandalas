//! Slider schemas per mandala + Common motion controls.
//! Values stored as f64 for uniformity; renderers cast as needed.

use crate::render::Mandala;

#[derive(Debug, Clone, Copy)]
pub struct Slider {
    pub key: &'static str,
    pub label: &'static str,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub def: f64,
    pub unit: &'static str,
}

impl Slider {
    pub fn format_value(&self, v: f64) -> String {
        if v.fract().abs() < 1e-9 {
            format!("{}{}", v as i64, self.unit)
        } else {
            format!("{:.2}{}", v, self.unit)
        }
    }
}

/// Per-mandala parameter values.
#[derive(Debug, Clone)]
pub struct Params {
    pub values: Vec<f64>,
}

impl Params {
    pub fn defaults(schema: &[Slider]) -> Self {
        Self { values: schema.iter().map(|s| s.def).collect() }
    }
    pub fn get(&self, idx: usize) -> f64 { self.values[idx] }
    pub fn set(&mut self, idx: usize, v: f64) { self.values[idx] = v; }
}

/// Global motion + look controls applied to every mandala.
#[derive(Debug, Clone)]
pub struct Common {
    pub speed: f64,
    pub pulse_depth: f64,
    pub pulse_rate: f64,
    pub hue_drift: f64,
    pub hue: f64,
    pub stroke: f64,
}

impl Default for Common {
    fn default() -> Self {
        Self {
            speed: 1.0,
            pulse_depth: 60.0,
            pulse_rate: 0.55,
            hue_drift: 0.0,
            hue: 0.0,
            stroke: 1.4,
        }
    }
}

pub const COMMON_SLIDERS: &[Slider] = &[
    Slider { key: "speed",       label: "Speed",        min: 0.0, max: 3.0,   step: 0.1,  def: 1.0,  unit: "x" },
    Slider { key: "pulse_depth", label: "Pulse depth",  min: 0.0, max: 100.0, step: 1.0,  def: 60.0, unit: "%" },
    Slider { key: "pulse_rate",  label: "Pulse rate",   min: 0.1, max: 2.0,   step: 0.05, def: 0.55, unit: "Hz" },
    Slider { key: "hue_drift",   label: "Hue drift",    min: 0.0, max: 60.0,  step: 1.0,  def: 0.0,  unit: "deg/s" },
    Slider { key: "hue",         label: "Hue shift",    min: 0.0, max: 360.0, step: 1.0,  def: 0.0,  unit: "deg" },
    Slider { key: "stroke",      label: "Stroke",       min: 0.5, max: 3.0,   step: 0.1,  def: 1.4,  unit: "" },
];

pub fn schema(m: Mandala) -> &'static [Slider] {
    match m {
        Mandala::Sacred => &[
            Slider { key: "rings",     label: "Concentric circles", min: 0.0, max: 8.0,  step: 1.0, def: 3.0,  unit: "" },
            Slider { key: "triangles", label: "Star pairs",         min: 0.0, max: 6.0,  step: 1.0, def: 2.0,  unit: "" },
            Slider { key: "scale",     label: "Inner star scale",   min: 20.0, max: 90.0, step: 1.0, def: 55.0, unit: "%" },
        ],
        Mandala::Lotus => &[
            Slider { key: "petals", label: "Outer petals", min: 4.0,  max: 18.0, step: 1.0, def: 8.0,  unit: "" },
            Slider { key: "layers", label: "Petal layers", min: 1.0,  max: 4.0,  step: 1.0, def: 2.0,  unit: "" },
            Slider { key: "length", label: "Petal length", min: 25.0, max: 80.0, step: 1.0, def: 50.0, unit: "" },
            Slider { key: "core",   label: "Core ring",    min: 4.0,  max: 30.0, step: 1.0, def: 12.0, unit: "" },
        ],
        Mandala::Spirograph => &[
            Slider { key: "R",    label: "Outer wheel R", min: 30.0, max: 90.0, step: 1.0, def: 60.0, unit: "" },
            Slider { key: "r",    label: "Inner wheel r", min: 4.0,  max: 50.0, step: 1.0, def: 18.0, unit: "" },
            Slider { key: "d",    label: "Pen offset d",  min: 5.0,  max: 60.0, step: 1.0, def: 30.0, unit: "" },
            Slider { key: "revs", label: "Revolutions",   min: 1.0,  max: 30.0, step: 1.0, def: 9.0,  unit: "" },
        ],
        Mandala::Star => &[
            Slider { key: "points", label: "Points per star",   min: 5.0,  max: 18.0, step: 1.0, def: 10.0, unit: "" },
            Slider { key: "nested", label: "Nested layers",     min: 1.0,  max: 5.0,  step: 1.0, def: 3.0,  unit: "" },
            Slider { key: "inner",  label: "Inner-point ratio", min: 20.0, max: 70.0, step: 1.0, def: 40.0, unit: "%" },
            Slider { key: "phase",  label: "Phase offset",      min: 0.0,  max: 90.0, step: 1.0, def: 18.0, unit: "deg" },
        ],
        Mandala::Flower => &[
            Slider { key: "layers", label: "Recursive layers", min: 1.0, max: 4.0,  step: 1.0, def: 3.0,  unit: "" },
            Slider { key: "radius", label: "Circle radius",    min: 15.0, max: 50.0, step: 1.0, def: 24.0, unit: "" },
        ],
        Mandala::Interlace => &[
            Slider { key: "rings",  label: "Rings",        min: 3.0,  max: 16.0, step: 1.0, def: 8.0,  unit: "" },
            Slider { key: "radius", label: "Ring radius",  min: 15.0, max: 55.0, step: 1.0, def: 30.0, unit: "" },
            Slider { key: "orbit",  label: "Orbit radius", min: 15.0, max: 60.0, step: 1.0, def: 38.0, unit: "" },
        ],
    }
}

/// Clamp `v` to the slider's range, snapped to `step` increments.
pub fn snap(s: &Slider, v: f64) -> f64 {
    let clamped = v.clamp(s.min, s.max);
    let steps = ((clamped - s.min) / s.step).round();
    (s.min + steps * s.step).clamp(s.min, s.max)
}

/// The virtual slider used by the Preset row in the sidebar.
pub const PRESET_SLIDER: Slider = Slider {
    key: "preset",
    label: "Preset",
    min: 0.0,
    max: 9.0,
    step: 1.0,
    def: 0.0,
    unit: "",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snap_clamps_above() {
        let s = COMMON_SLIDERS[0];
        assert_eq!(snap(&s, 99.0), 3.0);
    }

    #[test]
    fn snap_clamps_below() {
        let s = COMMON_SLIDERS[0];
        assert_eq!(snap(&s, -1.0), 0.0);
    }

    #[test]
    fn snap_to_step() {
        let s = COMMON_SLIDERS[0];
        let snapped = snap(&s, 1.04);
        assert!((snapped - 1.0).abs() < 1e-9);
    }

    #[test]
    fn schema_returns_all_mandalas() {
        for m in [Mandala::Sacred, Mandala::Lotus, Mandala::Spirograph, Mandala::Star, Mandala::Flower, Mandala::Interlace] {
            assert!(!schema(m).is_empty(), "schema missing for {:?}", m);
        }
    }
}
