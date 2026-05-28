pub mod canvas;
pub mod footer;
pub mod header;
pub mod sidebar;

use crate::app::AppState;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame as TuiFrame;

pub fn render(f: &mut TuiFrame, state: &AppState) {
    let area = f.area();

    // Vertical: header(2) / body(*) / footer(1)
    let vchunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(1),
        Constraint::Length(1),
    ]).split(area);

    header::render(f, vchunks[0], state);

    // Body: if wide enough and sidebar visible, split horizontally
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
}

/// Crop to the largest centered square that fits, accounting for terminal
/// cell aspect ratio (cells are ~2× taller than wide; doubling height yields
/// a visually square area).
fn square_center(area: Rect) -> Rect {
    let cell_aspect = 2.0_f32;
    let want_w = (area.height as f32 * cell_aspect) as u16;
    let w = want_w.min(area.width);
    let h = area.height;
    let x_off = area.x + area.width.saturating_sub(w) / 2;
    let y_off = area.y;
    Rect::new(x_off, y_off, w, h)
}
