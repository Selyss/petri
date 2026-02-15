use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // TODO: replace with real grid
    let status = if app.paused { "PAUSED" } else { "RUNNING" };
    let text = format!(
        "Gen: {}  |  {}  |  [space] pause  [n] step  [r] random  [c] clear  [q] quit",
        app.generation, status
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Game of Life ");

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}
