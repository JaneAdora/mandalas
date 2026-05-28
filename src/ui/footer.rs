use crate::app::AppState;
use crate::theme::{BG, DIM};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::Frame as TuiFrame;

fn rgb(c: (u8, u8, u8)) -> Color { Color::Rgb(c.0, c.1, c.2) }

pub fn render(f: &mut TuiFrame, area: Rect, state: &AppState) {
    let text = if let Some((msg, _)) = &state.toast {
        format!(" {msg}")
    } else {
        " Tab next · Space pause · ↑↓ pick · ←→ adjust · ? help · q quit".to_string()
    };
    f.render_widget(
        Paragraph::new(Span::styled(text, Style::default().fg(rgb(DIM)).bg(rgb(BG)))),
        area,
    );
}
