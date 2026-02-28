# Petri

A terminal-based Conway's Game of Life simulator built with Rust and [Ratatui](https://github.com/ratatui/ratatui). Features cell age visualization, adjustable zoom, pattern placement, and GIF recording.

## Building

```
cargo build --release
```

## Usage

```
cargo run --release
```

The simulation starts paused on an empty 256x256 grid. Use `r` to randomize, or toggle the cursor with `Tab` and draw cells manually with `Enter`. Use `p` to insert some design presets.

## Controls

| Key | Action |
|-----|--------|
| `Space` | Pause / resume |
| `n` | Step one generation |
| `r` | Randomize grid |
| `c` | Clear grid |
| `Tab` | Toggle cursor |
| `Enter` | Toggle cell at cursor |
| `h` `j` `k` `l` | Move cursor |
| `+` / `-` | Adjust simulation speed |
| `[` / `]` | Zoom out / in |
| `p` | Pattern mode |
| `1`-`3` | Place pattern (in pattern mode) |
| `g` | Start / stop GIF recording |
| `Esc` | Cancel pattern mode |
| `q` | Quit |

## Patterns

In pattern mode (`p`), the following patterns can be placed at the cursor position:

1. Glider
2. Pulsar
3. Gosper glider gun

## GIF Export

Press `g` to begin recording. Press `g` again to stop and save. Recordings are capped at 500 frames and auto-save when the limit is reached. Output files are written to the current directory as `petri_<timestamp>.gif`.
