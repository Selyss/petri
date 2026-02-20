use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.area());

    let mut lines: Vec<Line> = Vec::new();

    for y in 0..app.grid.height {
        let mut spans = Vec::new();
        for x in 0..app.grid.width {
            let alive = app.grid.cells[y * app.grid.width + x];
            let symbol = if alive { "██" } else { "  " };

            let style = if x == app.cursor_x && y == app.cursor_y && app.cursor_visible {
                Style::default().bg(Color::LightGreen)
            } else {
                Style::default()
            };
            spans.push(Span::styled(symbol, style));
        }
        lines.push(Line::from(spans))
    }

    let grid_block = Block::default().borders(Borders::ALL).title(" Petri ");

    let grid_widget = Paragraph::new(lines).block(grid_block);
    frame.render_widget(grid_widget, chunks[0]);

    let status = if app.paused { "PAUSED" } else { "RUNNING" };
    // TODO: add stats: alive/dead, speed, etc.

    let controls = format!(
        "Frame: {}  |  {}  | {}ms | [space] pause  [n] step  [r] random  [tab] cursor  [=/-] speed  [c] clear  [q] quit",
        app.generation,
        status,
        app.tick_rate.as_millis()
    );

    let status_block = Block::default().borders(Borders::ALL);

    let status_widget = Paragraph::new(controls).block(status_block);
    frame.render_widget(status_widget, chunks[1]);
}
