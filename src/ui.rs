use crate::app::App;
use crate::patterns;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
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

    let status = if app.paused { "PAUSED " } else { "RUNNING" };
    let status_style = if app.paused {
        Style::default()
            .bg(Color::Rgb(180, 60, 60))
            .fg(Color::White)
    } else {
        Style::default()
            .bg(Color::Rgb(60, 160, 60))
            .fg(Color::White)
    };

    let dim = Style::default()
        .bg(Color::Rgb(45, 45, 65))
        .fg(Color::Rgb(140, 140, 170));
    let bright = Style::default()
        .bg(Color::Rgb(60, 60, 90))
        .fg(Color::Rgb(200, 200, 230));
    let sep = Span::styled(" │ ", dim);

    let cursor_info = if app.cursor_visible {
        let cell_state = if app.grid.cells[app.cursor_y * app.grid.width + app.cursor_x] {
            "●"
        } else {
            "○"
        };
        vec![
            sep.clone(),
            Span::styled(
                format!(" ({},{}) {} ", app.cursor_x, app.cursor_y, cell_state),
                bright,
            ),
        ]
    } else {
        vec![]
    };

    let mode_spans = if app.pattern_mode {
        let pattern_list: String = patterns::ALL
            .iter()
            .enumerate()
            .map(|(i, p)| format!("{}:{}", i + 1, p.name))
            .collect::<Vec<_>>()
            .join(" ");
        vec![
            sep.clone(),
            Span::styled(
                format!(" PATTERN: {}  Esc:cancel ", pattern_list),
                Style::default()
                    .bg(Color::Rgb(160, 100, 40))
                    .fg(Color::White),
            ),
        ]
    } else {
        vec![]
    };

    let mut spans = vec![
        Span::styled(format!(" {} ", status), status_style),
        sep.clone(),
        Span::styled(format!(" Gen: {} ", app.generation), bright),
        sep.clone(),
        Span::styled(format!(" Alive: {} ", app.grid.population()), bright),
        sep.clone(),
        Span::styled(format!(" {}ms ", app.tick_rate.as_millis()), bright),
    ];
    spans.extend(cursor_info);
    spans.extend(mode_spans);
    spans.push(sep.clone());
    spans.push(Span::styled(
        " [spc] pause  [n] step  [r] rand  [tab] cursor  [±] speed  [c] clear  [q] quit ",
        dim,
    ));

    let status_line = Line::from(spans);
    let status_widget =
        Paragraph::new(status_line).style(Style::default().bg(Color::Rgb(45, 45, 65)));
    frame.render_widget(status_widget, chunks[1]);
}
