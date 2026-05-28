//! Stub — Task 21 implements the real config.
use crate::app::AppState;
use anyhow::Result;

#[derive(Debug, Default)]
pub struct Config;

impl Config {
    pub fn apply_to(&self, _state: &mut AppState) {}
}

pub fn load() -> Result<Config> {
    Err(anyhow::anyhow!("config not yet implemented"))
}
