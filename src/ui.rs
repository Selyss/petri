use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let status = if app.paused { "PAUSED" } else { "RUNNING" };
    let text = format!(
        "Gen: {}  |  {}  |  [space] pause  [n] step  [r] random  [c] clear  [q] quit",
        app.generation, status
    );

    let mut lines: Vec<Line> = Vec::new();

    for y in 0..app.grid.height {
        let mut row = String::new();
        for x in 0..app.grid.width {
            if app.grid.cells[y * app.grid.width + x] {
                row.push_str("██");
            } else {
                row.push_str("  ");
            }
        }
        lines.push(Line::from(row))
    }

    lines.push(Line::from(""));
    lines.push(Line::from(text));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Game of Life ");

    let paragraph = Paragraph::new(lines).block(block);

    frame.render_widget(paragraph, area);
}
