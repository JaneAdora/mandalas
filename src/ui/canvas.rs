use crate::app::AppState;
use crate::render::{self, Frame as RFrame};
use crate::theme::BG;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Points};
use ratatui::widgets::Block;
use ratatui::Frame as TuiFrame;

fn rgb(c: (u8, u8, u8)) -> Color { Color::Rgb(c.0, c.1, c.2) }

const HALF: f64 = 100.0;

pub fn render(f: &mut TuiFrame, area: Rect, state: &AppState) {
    let m = state.active;
    let mut frame = RFrame::default();
    render::render(m, state.current_params(), state.anim_time, &state.common, &mut frame);

    // Bucket points by color so we issue one Points draw per color
    let mut by_color: std::collections::HashMap<(u8, u8, u8), Vec<(f64, f64)>> = std::collections::HashMap::new();
    for p in &frame.points {
        by_color.entry(p.color).or_default().push((p.x, p.y));
    }

    let widget = Canvas::default()
        .block(Block::default().style(Style::default().bg(rgb(BG))))
        .marker(Marker::Braille)
        .x_bounds([-HALF, HALF])
        .y_bounds([-HALF, HALF])
        .paint(move |ctx| {
            for (col, pts) in by_color.iter() {
                ctx.draw(&Points { coords: pts.as_slice(), color: rgb(*col) });
            }
        });

    f.render_widget(widget, area);
}
