mod app;
mod grid;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};

use ratatui::prelude::*;

fn main() -> io::Result<()> {
    // custom panic because the terminal remains in raw mode otherwise

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        terminal::disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        default_panic(info);
    }));

    // setup
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    // cleanup
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = app::App::new(64, 32);
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
