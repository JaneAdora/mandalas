use crate::app::{AppState, SliderGroup};
use crate::controls::{schema, Slider, COMMON_SLIDERS};
use crate::theme::{BG, DIM, MAGENTA, TITLE, TEXT};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame as TuiFrame;

fn rgb(c: (u8, u8, u8)) -> Color { Color::Rgb(c.0, c.1, c.2) }

const BAR_WIDTH: usize = 14;

fn bar(v: f64, lo: f64, hi: f64) -> String {
    if hi <= lo { return " ".repeat(BAR_WIDTH); }
    let frac = ((v - lo) / (hi - lo)).clamp(0.0, 1.0);
    let filled = (frac * BAR_WIDTH as f64).round() as usize;
    let mut s = String::with_capacity(BAR_WIDTH);
    for i in 0..BAR_WIDTH {
        if i < filled { s.push('█'); } else { s.push('·'); }
    }
    s
}

fn format_value(v: f64, unit: &str) -> String {
    if v.fract().abs() < 1e-9 { format!("{}{}", v as i64, unit) }
    else { format!("{:.2}{}", v, unit) }
}

pub fn render(f: &mut TuiFrame, area: Rect, state: &AppState) {
    let mut lines: Vec<Line> = Vec::new();

    // Per-mandala section
    let mandala_focused = state.group == SliderGroup::Mandala;
    lines.push(Line::from(Span::styled(
        format!(" {}", state.active.name()),
        Style::default().fg(rgb(TITLE)).add_modifier(Modifier::ITALIC),
    )));
    for (i, s) in schema(state.active).iter().enumerate() {
        let val = state.current_params().get(i);
        let focused = mandala_focused && state.focus == i;
        lines.push(slider_line(s, val, focused));
    }

    lines.push(Line::from(""));

    // Common section
    let common_focused = state.group == SliderGroup::Common;
    lines.push(Line::from(Span::styled(
        " Motion / Look",
        Style::default().fg(rgb(TITLE)).add_modifier(Modifier::ITALIC),
    )));
    for (i, s) in COMMON_SLIDERS.iter().enumerate() {
        let val = match s.key {
            "speed"       => state.common.speed,
            "pulse_depth" => state.common.pulse_depth,
            "pulse_rate"  => state.common.pulse_rate,
            "hue_drift"   => state.common.hue_drift,
            "hue"         => state.common.hue,
            "stroke"      => state.common.stroke,
            _ => 0.0,
        };
        let focused = common_focused && state.focus == i;
        lines.push(slider_line(s, val, focused));
    }

    f.render_widget(
        Paragraph::new(lines)
            .block(Block::default()
                .borders(Borders::LEFT)
                .border_style(Style::default().fg(rgb(DIM))))
            .style(Style::default().bg(rgb(BG)).fg(rgb(TEXT))),
        area,
    );
}

fn slider_line(s: &Slider, v: f64, focused: bool) -> Line<'static> {
    let marker = if focused { "▸ " } else { "  " };
    let label_style = if focused {
        Style::default().fg(rgb(MAGENTA)).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(rgb(TEXT))
    };
    Line::from(vec![
        Span::raw(marker.to_string()),
        Span::styled(s.label.to_string(), label_style),
        Span::raw("  "),
        Span::styled(bar(v, s.min, s.max), Style::default().fg(rgb(MAGENTA))),
        Span::raw("  "),
        Span::styled(format_value(v, s.unit), Style::default().fg(rgb(DIM))),
    ])
}
