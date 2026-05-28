//! Config file at `~/.config/mandalas/config.toml`.

use crate::app::AppState;
use crate::controls::Common;
use crate::render::Mandala;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub default_mandala: Option<String>,
    pub motion: Option<MotionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionConfig {
    pub speed: Option<f64>,
    pub pulse_depth: Option<f64>,
    pub pulse_rate: Option<f64>,
    pub hue_drift: Option<f64>,
    pub hue: Option<f64>,
    pub stroke: Option<f64>,
}

impl Config {
    pub fn apply_to(&self, state: &mut AppState) {
        if let Some(slug) = &self.default_mandala {
            if let Some(m) = Mandala::from_slug(slug) {
                state.active = m;
            }
        }
        if let Some(m) = &self.motion {
            let mut c = Common::default();
            if let Some(v) = m.speed       { c.speed = v; }
            if let Some(v) = m.pulse_depth { c.pulse_depth = v; }
            if let Some(v) = m.pulse_rate  { c.pulse_rate = v; }
            if let Some(v) = m.hue_drift   { c.hue_drift = v; }
            if let Some(v) = m.hue         { c.hue = v; }
            if let Some(v) = m.stroke      { c.stroke = v; }
            state.common = c;
        }
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("mandalas")
        .join("config.toml")
}

pub fn load() -> Result<Config> {
    let path = config_path();
    let raw = std::fs::read_to_string(&path)?;
    Ok(toml::from_str(&raw)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_roundtrips() {
        let toml_src = r#"
default_mandala = "lotus"
[motion]
speed = 1.5
pulse_depth = 80
"#;
        let cfg: Config = toml::from_str(toml_src).unwrap();
        assert_eq!(cfg.default_mandala.as_deref(), Some("lotus"));
        assert_eq!(cfg.motion.unwrap().speed, Some(1.5));
    }
}
