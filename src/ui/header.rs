use crate::app::AppState;
use crate::theme::{BG, DIM, MAGENTA, MUSTARD, TITLE};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame as TuiFrame;

fn rgb(c: (u8, u8, u8)) -> Color { Color::Rgb(c.0, c.1, c.2) }

pub fn render(f: &mut TuiFrame, area: Rect, state: &AppState) {
    // Line 1: " mandalas · <Mandala Name>   [PAUSED if paused]"
    let mut title_spans = vec![
        Span::styled(" mandalas", Style::default().fg(rgb(TITLE)).add_modifier(Modifier::BOLD)),
        Span::styled(" · ", Style::default().fg(rgb(DIM))),
        Span::styled(state.active.name(), Style::default().fg(rgb(TITLE)).add_modifier(Modifier::ITALIC)),
    ];
    if state.paused {
        title_spans.push(Span::styled("   ⏸ PAUSED", Style::default().fg(rgb(MUSTARD)).add_modifier(Modifier::BOLD)));
    }
    let title = Line::from(title_spans);

    // Line 2: " ▸ <focused slider label>: <value><unit>"
    let (sl, v) = state.focused_slider();
    let focus = Line::from(vec![
        Span::styled(" ▸ ", Style::default().fg(rgb(MAGENTA)).add_modifier(Modifier::BOLD)),
        Span::styled(sl.label.to_string(), Style::default().fg(rgb(MAGENTA)).add_modifier(Modifier::BOLD)),
        Span::styled(": ", Style::default().fg(rgb(DIM))),
        Span::styled(sl.format_value(v), Style::default().fg(rgb(MAGENTA))),
    ]);

    f.render_widget(
        Paragraph::new(vec![title, focus]).style(Style::default().bg(rgb(BG))),
        area,
    );
}
