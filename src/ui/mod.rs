pub mod canvas;
pub mod footer;
pub mod header;
pub mod sidebar;

use crate::app::AppState;
use crate::theme::{BG, MAGENTA, TEXT, TITLE};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame as TuiFrame;

fn rgb(c: (u8, u8, u8)) -> Color { Color::Rgb(c.0, c.1, c.2) }

pub fn render(f: &mut TuiFrame, state: &AppState) {
    let area = f.area();

    let vchunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(1),
        Constraint::Length(1),
    ]).split(area);

    header::render(f, vchunks[0], state);

    let body = vchunks[1];
    let wide = body.width >= 80;
    if wide && state.sidebar_visible {
        let hchunks = Layout::horizontal([
            Constraint::Min(20),
            Constraint::Length(36),
        ]).split(body);
        canvas::render(f, square_center(hchunks[0]), state);
        sidebar::render(f, hchunks[1], state);
    } else {
        canvas::render(f, square_center(body), state);
    }

    footer::render(f, vchunks[2], state);

    if state.help_open {
        render_help_modal(f, area);
    }
}

fn square_center(area: Rect) -> Rect {
    let cell_aspect = 2.0_f32;
    let want_w = (area.height as f32 * cell_aspect) as u16;
    let w = want_w.min(area.width);
    let h = area.height;
    let x_off = area.x + area.width.saturating_sub(w) / 2;
    let y_off = area.y;
    Rect::new(x_off, y_off, w, h)
}

fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let x = area.x + area.width.saturating_sub(w) / 2;
    let y = area.y + area.height.saturating_sub(h) / 2;
    Rect::new(x, y, w.min(area.width), h.min(area.height))
}

fn render_help_modal(f: &mut TuiFrame, area: Rect) {
    let modal = centered(area, 56, 16);
    let lines = vec![
        Line::from(Span::styled(
            " Keys",
            Style::default().fg(rgb(TITLE)).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(" Tab / Shift-Tab     cycle mandalas"),
        Line::from(" ↑ / ↓               move focus"),
        Line::from(" ← / →               adjust slider (Shift = 10×)"),
        Line::from(" Space               pause / resume"),
        Line::from(" H                   toggle sidebar"),
        Line::from(" R                   randomize current"),
        Line::from(" s                   save preset"),
        Line::from(" 1-9                 load preset"),
        Line::from(" ?                   toggle this help"),
        Line::from(" q / Esc             quit"),
    ];
    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(rgb(MAGENTA))),
        )
        .style(Style::default().bg(rgb(BG)).fg(rgb(TEXT)));
    f.render_widget(Clear, modal);
    f.render_widget(p, modal);
}
