use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use mandalas::app::AppState;
use mandalas::{config, presets, ui};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::time::Duration;

fn main() -> Result<()> {
    // Panic hook restores terminal so a crash doesn't leave the term broken
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = io::stdout().execute(LeaveAlternateScreen);
        let _ = io::stdout().execute(DisableMouseCapture);
        prev(info);
    }));

    enable_raw_mode()?;
    let mut out = io::stdout();
    out.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(out);
    let mut term = Terminal::new(backend)?;

    let mut state = AppState::new();
    if let Ok(cfg) = config::load() {
        cfg.apply_to(&mut state);
    }
    state.presets = presets::load().unwrap_or_default();

    let result = run_loop(&mut term, &mut state);

    let _ = disable_raw_mode();
    let _ = io::stdout().execute(LeaveAlternateScreen);
    let _ = io::stdout().execute(DisableMouseCapture);
    result
}

fn run_loop(term: &mut Terminal<CrosstermBackend<Stdout>>, state: &mut AppState) -> Result<()> {
    loop {
        state.tick();
        term.draw(|f| ui::render(f, state))?;
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(k) = event::read()? {
                if k.kind != KeyEventKind::Press { continue; }
                let shift = k.modifiers.contains(KeyModifiers::SHIFT);
                let multiplier = if shift { 10.0 } else { 1.0 };
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Tab     => state.next_mandala(),
                    KeyCode::BackTab => state.prev_mandala(),
                    KeyCode::Up      => state.focus_up(),
                    KeyCode::Down    => state.focus_down(),
                    KeyCode::Left    => state.adjust(-multiplier),
                    KeyCode::Right   => state.adjust(multiplier),
                    KeyCode::Char(' ') => state.toggle_pause(),
                    KeyCode::Char('H') => state.toggle_sidebar(),
                    KeyCode::Char('R') => state.randomize_active(),
                    KeyCode::Char('?') => state.toggle_help(),
                    KeyCode::Char('0') => {
                        state.preset_slot = 0;
                        state.apply_preset_slot(0);
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let slot = c.to_digit(10).unwrap() as u8;
                        state.preset_slot = slot;
                        state.apply_preset_slot(slot);
                    }
                    KeyCode::Char('s') => {
                        let target_slot = if state.preset_slot > 0 && !state.presets.contains_key(&state.preset_slot) {
                            // user landed on an empty slot and wants to save there
                            state.preset_slot
                        } else if state.preset_slot > 0 {
                            // user is on a saved slot — overwrite
                            state.preset_slot
                        } else {
                            // slot 0 (reset) — fall through to next empty
                            next_empty_slot(state).unwrap_or(1)
                        };
                        let preset = presets::Preset::from_state(state, format!("slot{target_slot}"));
                        state.presets.insert(target_slot, preset.clone());
                        let _ = presets::save(&state.presets);
                        state.preset_slot = target_slot;
                        state.show_toast(format!("saved to preset {target_slot}"));
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn next_empty_slot(state: &AppState) -> Option<u8> {
    (1u8..=9).find(|s| !state.presets.contains_key(s))
}
