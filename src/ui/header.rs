use crate::app::AppState;
use crate::theme::{BG, DIM, TITLE};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Modifier};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame as TuiFrame;

fn rgb(c: (u8, u8, u8)) -> Color { Color::Rgb(c.0, c.1, c.2) }

pub fn render(f: &mut TuiFrame, area: Rect, state: &AppState) {
    let title = Line::from(vec![
        Span::styled(" mandalas", Style::default().fg(rgb(TITLE)).add_modifier(Modifier::BOLD)),
        Span::styled(" · ", Style::default().fg(rgb(DIM))),
        Span::styled(state.active.name(), Style::default().fg(rgb(TITLE)).add_modifier(Modifier::ITALIC)),
    ]);
    let meta = Line::from(vec![
        Span::styled(
            format!(" {} · t={:.1}s{}",
                state.active.slug(),
                state.anim_time,
                if state.paused { " · PAUSED" } else { "" }),
            Style::default().fg(rgb(DIM)),
        ),
    ]);
    f.render_widget(
        Paragraph::new(vec![title, meta]).style(Style::default().bg(rgb(BG))),
        area,
    );
}
