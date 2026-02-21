mod app;
mod export;
mod grid;
mod patterns;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Instant};

use ratatui::prelude::*;

fn main() -> io::Result<()> {
    // custom panic because the terminal remains in raw mode otherwise
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        terminal::disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        default_panic(info);
    }));

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let (cols, rows) = crossterm::terminal::size()?;
    let view_width = (cols as usize - 2) / 2;
    let view_height = rows as usize - 3;

    let mut app = app::App::new(256, 256, view_width, view_height);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        let timeout = app.tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => app.toggle_pause(),
                    KeyCode::Char('n') => app.step(),
                    KeyCode::Char('r') => app.randomize(),
                    KeyCode::Char('c') => app.clear(),
                    KeyCode::Enter => app.toggle_cell(),
                    KeyCode::Char('h') => app.move_left(),
                    KeyCode::Char('j') => app.move_down(),
                    KeyCode::Char('k') => app.move_up(),
                    KeyCode::Char('l') => app.move_right(),
                    KeyCode::Char('+') | KeyCode::Char('=') => app.speed_up(),
                    KeyCode::Char('-') => app.slow_down(),
                    KeyCode::Tab => app.toggle_cursor(),
                    KeyCode::Char(']') => app.zoom_in(),
                    KeyCode::Char('[') => app.zoom_out(),
                    KeyCode::Char('p') => {
                        app.pattern_mode = !app.pattern_mode;
                        if app.pattern_mode && !app.cursor_visible {
                            app.cursor_visible = true;
                        }
                    }
                    KeyCode::Char(c) if app.pattern_mode && c.is_ascii_digit() => {
                        let idx = c.to_digit(10).unwrap() as usize;
                        if idx >= 1 && idx <= patterns::ALL.len() {
                            app.place_pattern(patterns::ALL[idx - 1]);
                            app.pattern_mode = false;
                        }
                    }
                    KeyCode::Char('g') => {
                        if app.recording {
                            let frames = app.stop_recording();
                            let tick_ms = app.tick_rate.as_millis() as u16;
                            match export::encode_gif(
                                &frames,
                                app.grid.width,
                                app.grid.height,
                                4,
                                tick_ms,
                            ) {
                                Ok(filename) => {
                                    app.last_export_msg =
                                        Some(format!("Saved {} ({} frames)", filename, frames.len()));
                                }
                                Err(e) => {
                                    app.last_export_msg = Some(format!("Export failed: {}", e));
                                }
                            }
                        } else {
                            app.start_recording();
                            app.last_export_msg = None;
                        }
                    }
                    KeyCode::Esc => {
                        app.pattern_mode = false;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= app.tick_rate {
            if !app.paused {
                app.step();
            }
            last_tick = Instant::now();
        }
    }
}
