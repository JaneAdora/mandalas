//! Stub — Task 22 implements the real preset module.

use crate::app::AppState;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
}

impl Preset {
    pub fn from_state(_state: &AppState, name: String) -> Self {
        Self { name }
    }
    pub fn apply_to(self, _state: &mut AppState) {}
}

pub fn load() -> Result<HashMap<u8, Preset>> {
    Ok(HashMap::new())
}

pub fn save(_presets: &HashMap<u8, Preset>) -> Result<()> {
    Ok(())
}
