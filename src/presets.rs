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

    /// Apply this preset to state. Does NOT switch the active mandala
    /// (per-style presets stay on the user's current mandala). Only sets
    /// per-mandala params + Common motion.
    pub fn apply_to(self, state: &mut AppState) {
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
struct PresetFile {
    presets: HashMap<String, HashMap<u8, Preset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct OldPresetFile {
    presets: HashMap<u8, Preset>,
}

fn presets_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("mandalas")
        .join("presets.toml")
}

pub fn load() -> Result<HashMap<Mandala, HashMap<u8, Preset>>> {
    let path = presets_path();
    if !path.exists() { return Ok(HashMap::new()); }
    let raw = std::fs::read_to_string(&path)?;

    // Try new (nested) format first
    if let Ok(file) = toml::from_str::<PresetFile>(&raw) {
        let mut out: HashMap<Mandala, HashMap<u8, Preset>> = HashMap::new();
        for (slug, slots) in file.presets {
            if let Some(m) = Mandala::from_slug(&slug) {
                out.insert(m, slots);
            }
        }
        return Ok(out);
    }

    // Fall back to old flat format and migrate by grouping on preset.mandala
    let old: OldPresetFile = toml::from_str(&raw)?;
    let mut out: HashMap<Mandala, HashMap<u8, Preset>> = HashMap::new();
    for (slot, preset) in old.presets {
        if let Some(m) = Mandala::from_slug(&preset.mandala) {
            out.entry(m).or_default().insert(slot, preset);
        }
    }
    Ok(out)
}

pub fn save(presets: &HashMap<Mandala, HashMap<u8, Preset>>) -> Result<()> {
    let path = presets_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut by_slug: HashMap<String, HashMap<u8, Preset>> = HashMap::new();
    for (m, slots) in presets {
        by_slug.insert(m.slug().to_string(), slots.clone());
    }
    let file = PresetFile { presets: by_slug };
    let s = toml::to_string_pretty(&file)?;
    std::fs::write(&path, s)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Mandala;

    #[test]
    fn preset_does_not_change_active_mandala() {
        // After per-style refactor: loading a preset should NOT swap the
        // active mandala (presets are scoped to whatever mandala you're on).
        let mut state = AppState::new();
        state.active = Mandala::Lotus;
        let pre = Preset::from_state(&state, "test".into());
        state.active = Mandala::Sacred;
        pre.apply_to(&mut state);
        assert_eq!(state.active, Mandala::Sacred, "active should not change");
    }
}
