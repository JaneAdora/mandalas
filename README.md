# mandalas

Animated mandalas in your terminal, purely for visual joy. Sibling of `wt` / `recall` / `roam` / `glance` / `atlas` / `cal`.

Six mandala styles — Sacred Geometry, Lotus, Spirograph, Star Lattice, Flower of Life, Interlace — each with bespoke animation behaviour (echo trails, blooming petals, ripple waves, counter-rotation). Per-mandala parameters + global motion controls (speed, pulse depth/rate, hue drift, manual hue, stroke). Pause, save presets, load by number.

## Build

```
cd ~/projects/mandalas
./install.sh
```

Installs to `~/.cargo/bin/mandalas` and `~/.local/bin/mandalas` (whichever wins PATH wins).

## Usage

```
mandalas         # launch with default mandala
```

## Keys

| Key | Action |
|---|---|
| `Tab` / `Shift+Tab` | cycle mandalas |
| `↑` / `↓` | move focus between sliders |
| `←` / `→` | adjust focused slider (Shift = 10× steps) |
| `Space` | pause/resume |
| `H` | toggle sidebar |
| `R` | randomize current mandala's params |
| `s` | save current state to next empty preset slot |
| `1`-`9` | load preset |
| `?` | toggle help |
| `q` / `Esc` | quit |

## Config

`~/.config/mandalas/config.toml` (all optional):

```toml
default_mandala = "lotus"

[motion]
speed = 1.0
pulse_depth = 60
pulse_rate = 0.55
hue_drift = 0
hue = 0
stroke = 1.4
```

Presets live at `~/.config/mandalas/presets.toml` and are managed in-app with `s` (save) / `1`–`9` (load).

## Terminal requirements

- Truecolor (24-bit) for smooth hue cycling — standard on modern terminals.
- Braille font support for the canvas marker — Termux, kitty, alacritty, iTerm2, ghostty all work.

## Why

For visual joy.
