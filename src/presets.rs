//! Numbered presets at `~/.config/mandalas/presets.toml`.

use crate::app::AppState;
use crate::controls::Common;
use crate::render::Mandala;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub mandala: String,
    pub params: Vec<f64>,
    pub speed: f64,
    pub pulse_depth: f64,
    pub pulse_rate: f64,
    pub hue_drift: f64,
    pub hue: f64,
    pub stroke: f64,
}

impl Preset {
    pub fn from_state(state: &AppState, name: String) -> Self {
        let m = state.active;
        Self {
            name,
            mandala: m.slug().to_string(),
            params: state.current_params().values.clone(),
            speed: state.common.speed,
            pulse_depth: state.common.pulse_depth,
            pulse_rate: state.common.pulse_rate,
            hue_drift: state.common.hue_drift,
            hue: state.common.hue,
            stroke: state.common.stroke,
        }
    }

    pub fn apply_to(self, state: &mut AppState) {
        if let Some(m) = Mandala::from_slug(&self.mandala) {
            state.active = m;
        }
        let s = crate::controls::schema(state.active);
        if self.params.len() == s.len() {
            state.current_params_mut().values = self.params;
        }
        state.common = Common {
            speed: self.speed,
            pulse_depth: self.pulse_depth,
            pulse_rate: self.pulse_rate,
            hue_drift: self.hue_drift,
            hue: self.hue,
            stroke: self.stroke,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PresetFile { presets: HashMap<u8, Preset> }

fn presets_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("mandalas")
        .join("presets.toml")
}

pub fn load() -> Result<HashMap<u8, Preset>> {
    let path = presets_path();
    if !path.exists() { return Ok(HashMap::new()); }
    let raw = std::fs::read_to_string(&path)?;
    let file: PresetFile = toml::from_str(&raw)?;
    Ok(file.presets)
}

pub fn save(presets: &HashMap<u8, Preset>) -> Result<()> {
    let path = presets_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = PresetFile { presets: presets.clone() };
    let s = toml::to_string_pretty(&file)?;
    std::fs::write(&path, s)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preset_roundtrips_in_memory() {
        let mut state = AppState::new();
        let original_active = state.active;
        let pre = Preset::from_state(&state, "test".into());
        state.active = state.active.next();
        pre.apply_to(&mut state);
        assert_eq!(state.active, original_active);
    }
}
