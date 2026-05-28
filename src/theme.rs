//! Editorial palette shared with the rest of the suite.
//! Tuples are (r, g, b) so renderers can pass them to `ratatui::style::Color::Rgb`.

pub const ROSE:     (u8, u8, u8) = (0xe8, 0x8b, 0x9f);
pub const LAVENDER: (u8, u8, u8) = (0xc5, 0xa3, 0xff);
pub const MUSTARD:  (u8, u8, u8) = (0xd9, 0xa4, 0x41);
pub const SAGE:     (u8, u8, u8) = (0x9b, 0xb5, 0x9a);
pub const MAGENTA:  (u8, u8, u8) = (0xff, 0x6e, 0xc7);

pub const BG:       (u8, u8, u8) = (0x0a, 0x08, 0x15);
pub const DIM:      (u8, u8, u8) = (0x80, 0x76, 0x9a);
pub const TEXT:     (u8, u8, u8) = (0xcf, 0xc8, 0xde);
pub const TITLE:    (u8, u8, u8) = (0xc5, 0xa3, 0xff);

pub const ALL: &[(u8, u8, u8)] = &[ROSE, LAVENDER, MUSTARD, SAGE, MAGENTA];
