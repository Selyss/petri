use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let status = if app.paused { "PAUSED" } else { "RUNNING" };
    let text = format!(
        "Frame: {}  |  {}  |  [space] pause  [n] step  [r] random  [c] clear  [q] quit",
        app.generation, status
    );

    let mut lines: Vec<Line> = Vec::new();

    for y in 0..app.grid.height {
        let mut spans = Vec::new();
        for x in 0..app.grid.width {
            let alive = app.grid.cells[y * app.grid.width + x];
            let symbol = if alive { "██" } else { "  " };

            let style = if x == app.cursor_x && y == app.cursor_y {
                Style::default().bg(Color::LightGreen)
            } else {
                Style::default()
            };
            spans.push(Span::styled(symbol, style));
        }
        lines.push(Line::from(spans))
    }

    lines.push(Line::from(""));
    lines.push(Line::from(text));

    let block = Block::default().borders(Borders::ALL).title(" Petri ");

    let paragraph = Paragraph::new(lines).block(block);

    frame.render_widget(paragraph, area);
}
