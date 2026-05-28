//! AppState: which mandala is active, slider focus, params per mandala, common, pause state.

use crate::controls::{schema, snap, Common, Params, Slider, COMMON_SLIDERS, PRESET_SLIDER};
use crate::render::Mandala;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderGroup { Preset, Mandala, Common }

pub struct AppState {
    pub active: Mandala,
    pub params: HashMap<Mandala, Params>,
    pub common: Common,
    pub group: SliderGroup,
    pub focus: usize,
    pub preset_slot: u8,
    pub paused: bool,
    pub sidebar_visible: bool,
    pub help_open: bool,
    pub start: Instant,
    pub anim_time: f64,
    pub last_tick: Instant,
    pub presets: HashMap<u8, crate::presets::Preset>,
    pub toast: Option<(String, Instant)>,
}

impl AppState {
    pub fn new() -> Self {
        let mut params = HashMap::new();
        for m in Mandala::ALL {
            params.insert(*m, Params::defaults(schema(*m)));
        }
        Self {
            active: Mandala::Sacred,
            params,
            common: Common::default(),
            group: SliderGroup::Mandala,
            focus: 0,
            preset_slot: 0,
            paused: false,
            sidebar_visible: true,
            help_open: false,
            start: Instant::now(),
            anim_time: 0.0,
            last_tick: Instant::now(),
            presets: HashMap::new(),
            toast: None,
        }
    }

    pub fn current_params(&self) -> &Params { &self.params[&self.active] }
    pub fn current_params_mut(&mut self) -> &mut Params { self.params.get_mut(&self.active).unwrap() }

    pub fn next_mandala(&mut self) { self.active = self.active.next(); self.focus = 0; self.group = SliderGroup::Mandala; }
    pub fn prev_mandala(&mut self) { self.active = self.active.prev(); self.focus = 0; self.group = SliderGroup::Mandala; }

    pub fn focus_down(&mut self) {
        let group_len = self.group_len();
        if self.focus + 1 < group_len {
            self.focus += 1;
        } else {
            match self.group {
                SliderGroup::Preset  => { self.group = SliderGroup::Mandala; self.focus = 0; }
                SliderGroup::Mandala => { self.group = SliderGroup::Common; self.focus = 0; }
                SliderGroup::Common  => {}
            }
        }
    }

    pub fn focus_up(&mut self) {
        if self.focus > 0 {
            self.focus -= 1;
        } else {
            match self.group {
                SliderGroup::Preset  => {}
                SliderGroup::Mandala => { self.group = SliderGroup::Preset; self.focus = 0; }
                SliderGroup::Common  => {
                    self.group = SliderGroup::Mandala;
                    self.focus = schema(self.active).len().saturating_sub(1);
                }
            }
        }
    }

    fn group_len(&self) -> usize {
        match self.group {
            SliderGroup::Preset  => 1,
            SliderGroup::Mandala => schema(self.active).len(),
            SliderGroup::Common  => COMMON_SLIDERS.len(),
        }
    }

    pub fn adjust(&mut self, delta_steps: f64) {
        match self.group {
            SliderGroup::Preset => {
                let new_raw = (self.preset_slot as i32) + (delta_steps as i32);
                let new_slot = new_raw.clamp(0, 9) as u8;
                if new_slot != self.preset_slot {
                    self.preset_slot = new_slot;
                    self.apply_preset_slot(new_slot);
                }
            }
            SliderGroup::Mandala => {
                let s = schema(self.active)[self.focus];
                let cur = self.current_params().get(self.focus);
                let new = snap(&s, cur + delta_steps * s.step);
                let focus = self.focus;
                self.current_params_mut().set(focus, new);
            }
            SliderGroup::Common => {
                let s = COMMON_SLIDERS[self.focus];
                let delta = delta_steps * s.step;
                match s.key {
                    "speed"       => self.common.speed       = snap(&s, self.common.speed + delta),
                    "pulse_depth" => self.common.pulse_depth = snap(&s, self.common.pulse_depth + delta),
                    "pulse_rate"  => self.common.pulse_rate  = snap(&s, self.common.pulse_rate + delta),
                    "hue_drift"   => self.common.hue_drift   = snap(&s, self.common.hue_drift + delta),
                    "hue"         => self.common.hue         = snap(&s, self.common.hue + delta),
                    "stroke"      => self.common.stroke      = snap(&s, self.common.stroke + delta),
                    _ => {}
                }
            }
        }
    }

    /// Apply the chosen preset slot: 0 resets, 1-9 loads if saved, else no-op.
    pub fn apply_preset_slot(&mut self, slot: u8) {
        if slot == 0 {
            self.reset_to_defaults();
            self.show_toast("reset to defaults");
            return;
        }
        if let Some(preset) = self.presets.get(&slot).cloned() {
            preset.apply_to(self);
            self.normalize_focus();
            self.show_toast(format!("preset {slot} loaded"));
        } else {
            self.show_toast(format!("preset {slot} empty"));
        }
    }

    /// Reset all per-mandala params to schema defaults and Common to Common::default().
    /// Active mandala is preserved.
    pub fn reset_to_defaults(&mut self) {
        for m in Mandala::ALL {
            self.params.insert(*m, Params::defaults(schema(*m)));
        }
        self.common = Common::default();
    }

    /// Clamp focus to within the current group_len after any operation that
    /// could leave it out of bounds (e.g., loading a preset whose mandala has
    /// fewer sliders than the current focus).
    pub fn normalize_focus(&mut self) {
        let len = self.group_len();
        if len == 0 { return; }
        if self.focus >= len { self.focus = len - 1; }
    }

    /// Human-readable status for a preset slot, used in the sidebar/header.
    pub fn preset_status_label(&self, slot: u8) -> String {
        if slot == 0 {
            "RESET".to_string()
        } else if self.presets.contains_key(&slot) {
            format!("★ slot {}", slot)
        } else {
            format!("[empty] slot {}", slot)
        }
    }

    /// The (slider, current value) pair the user is currently focused on.
    pub fn focused_slider(&self) -> (Slider, f64) {
        match self.group {
            SliderGroup::Preset => (PRESET_SLIDER, self.preset_slot as f64),
            SliderGroup::Mandala => {
                let s = schema(self.active);
                let sl = s[self.focus];
                let v = self.current_params().get(self.focus);
                (sl, v)
            }
            SliderGroup::Common => {
                let sl = COMMON_SLIDERS[self.focus];
                let v = match sl.key {
                    "speed"       => self.common.speed,
                    "pulse_depth" => self.common.pulse_depth,
                    "pulse_rate"  => self.common.pulse_rate,
                    "hue_drift"   => self.common.hue_drift,
                    "hue"         => self.common.hue,
                    "stroke"      => self.common.stroke,
                    _ => 0.0,
                };
                (sl, v)
            }
        }
    }

    pub fn toggle_pause(&mut self)   { self.paused = !self.paused; }
    pub fn toggle_sidebar(&mut self) { self.sidebar_visible = !self.sidebar_visible; }
    pub fn toggle_help(&mut self)    { self.help_open = !self.help_open; }

    /// Advance animation clock; auto-drift hue.
    pub fn tick(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f64().min(0.05);
        self.last_tick = now;
        if !self.paused {
            self.anim_time += dt;
            if self.common.hue_drift > 0.0 {
                self.common.hue = (self.common.hue + dt * self.common.hue_drift).rem_euclid(360.0);
            }
        }
        if let Some((_, ts)) = self.toast {
            if now.duration_since(ts).as_secs() >= 3 { self.toast = None; }
        }
    }

    pub fn randomize_active(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let s = schema(self.active);
        let mut vals = Vec::with_capacity(s.len());
        for sl in s {
            let raw = rng.gen_range(sl.min..=sl.max);
            vals.push(snap(sl, raw));
        }
        self.current_params_mut().values = vals;
        self.toast = Some(("randomised".into(), Instant::now()));
    }

    pub fn show_toast(&mut self, msg: impl Into<String>) {
        self.toast = Some((msg.into(), Instant::now()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_down_walks_into_common() {
        let mut s = AppState::new();
        let m_len = schema(s.active).len();
        for _ in 0..(m_len - 1) { s.focus_down(); }
        assert_eq!(s.group, SliderGroup::Mandala);
        s.focus_down();
        assert_eq!(s.group, SliderGroup::Common);
        assert_eq!(s.focus, 0);
    }

    #[test]
    fn next_mandala_cycles() {
        let mut s = AppState::new();
        for _ in 0..Mandala::ALL.len() { s.next_mandala(); }
        assert_eq!(s.active, Mandala::Sacred);
    }

    #[test]
    fn adjust_changes_param() {
        let mut s = AppState::new();
        let before = s.current_params().get(0);
        s.adjust(1.0);
        let after = s.current_params().get(0);
        assert!((after - before).abs() > 1e-9 || (before == schema(s.active)[0].max));
    }

    #[test]
    fn focus_walk_starts_at_preset() {
        let mut s = AppState::new();
        // Default focus starts at Mandala (group = Mandala, focus = 0).
        // focus_up should land on Preset.
        s.focus_up();
        assert_eq!(s.group, SliderGroup::Preset);
        assert_eq!(s.focus, 0);
    }

    #[test]
    fn reset_to_defaults_restores_common() {
        let mut s = AppState::new();
        s.common.speed = 2.5;
        s.common.hue = 180.0;
        s.reset_to_defaults();
        let def = Common::default();
        assert!((s.common.speed - def.speed).abs() < 1e-9);
        assert!((s.common.hue - def.hue).abs() < 1e-9);
    }

    #[test]
    fn apply_preset_slot_zero_resets() {
        let mut s = AppState::new();
        s.common.speed = 2.5;
        s.apply_preset_slot(0);
        assert!((s.common.speed - Common::default().speed).abs() < 1e-9);
    }
}
