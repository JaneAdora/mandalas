//! AppState: which mandala is active, slider focus, params per mandala, common, pause state.

use crate::controls::{schema, snap, Common, Params, COMMON_SLIDERS};
use crate::render::Mandala;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderGroup { Mandala, Common }

pub struct AppState {
    pub active: Mandala,
    pub params: HashMap<Mandala, Params>,
    pub common: Common,
    pub group: SliderGroup,
    pub focus: usize,
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
        } else if self.group == SliderGroup::Mandala {
            self.group = SliderGroup::Common;
            self.focus = 0;
        }
    }

    pub fn focus_up(&mut self) {
        if self.focus > 0 {
            self.focus -= 1;
        } else if self.group == SliderGroup::Common {
            self.group = SliderGroup::Mandala;
            self.focus = schema(self.active).len().saturating_sub(1);
        }
    }

    fn group_len(&self) -> usize {
        match self.group {
            SliderGroup::Mandala => schema(self.active).len(),
            SliderGroup::Common => COMMON_SLIDERS.len(),
        }
    }

    pub fn adjust(&mut self, delta_steps: f64) {
        match self.group {
            SliderGroup::Mandala => {
                let focus = self.focus;
                let s = schema(self.active)[focus];
                let cur = self.current_params().get(focus);
                let new = snap(&s, cur + delta_steps * s.step);
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
}
