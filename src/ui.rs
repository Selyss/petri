use crate::app::App;
use crate::patterns;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

fn age_color(age: u16) -> Color {
    match age {
        0 => Color::Reset,
        1 => Color::Rgb(40, 80, 40),
        2..=5 => Color::Rgb(60, 140, 60),
        6..=15 => Color::Rgb(100, 180, 50),
        16..=40 => Color::Rgb(180, 180, 40),
        41..=100 => Color::Rgb(220, 140, 30),
        _ => Color::Rgb(240, 240, 240),
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let mut lines: Vec<Line> = Vec::new();

    if app.zoom == 0 {
        // Half-block mode: 1 char per cell, 2 grid rows per terminal row
        let visible_cols = app.view_width * 2;
        let visible_rows = app.view_height * 2;

        for row_pair in (0..visible_rows).step_by(2) {
            let y_top = app.viewport_y + row_pair;
            let y_bot = app.viewport_y + row_pair + 1;
            let mut spans = Vec::new();

            for col in 0..visible_cols {
                let x = app.viewport_x + col;
                if x >= app.grid.width {
                    break;
                }
                let age_top = if y_top < app.grid.height {
                    app.grid.cells[y_top * app.grid.width + x]
                } else {
                    0
                };
                let age_bot = if y_bot < app.grid.height {
                    app.grid.cells[y_bot * app.grid.width + x]
                } else {
                    0
                };

                let is_cursor_top =
                    app.cursor_visible && x == app.cursor_x && y_top == app.cursor_y;
                let is_cursor_bot =
                    app.cursor_visible && x == app.cursor_x && y_bot == app.cursor_y;

                if is_cursor_top || is_cursor_bot {
                    spans.push(Span::styled(
                        "▀",
                        Style::default()
                            .fg(if is_cursor_top {
                                Color::LightGreen
                            } else if age_top > 0 {
                                age_color(age_top)
                            } else {
                                Color::Reset
                            })
                            .bg(if is_cursor_bot {
                                Color::LightGreen
                            } else if age_bot > 0 {
                                age_color(age_bot)
                            } else {
                                Color::Reset
                            }),
                    ));
                } else {
                    let (ch, style) = match (age_top > 0, age_bot > 0) {
                        (true, true) => (
                            "▀",
                            Style::default()
                                .fg(age_color(age_top))
                                .bg(age_color(age_bot)),
                        ),
                        (true, false) => {
                            ("▀", Style::default().fg(age_color(age_top)))
                        }
                        (false, true) => {
                            ("▄", Style::default().fg(age_color(age_bot)))
                        }
                        (false, false) => (" ", Style::default()),
                    };
                    spans.push(Span::styled(ch, style));
                }
            }
            lines.push(Line::from(spans));
        }
    } else {
        let zoom = app.zoom as usize;
        let visible_cols = app.view_width / zoom;
        let visible_rows = app.view_height / zoom;
        let cell_str: String = "██".repeat(zoom);
        let empty_str: String = "  ".repeat(zoom);
        for y in app.viewport_y..app.viewport_y + visible_rows {
            let mut spans = Vec::new();
            for x in app.viewport_x..app.viewport_x + visible_cols {
                let age = app.grid.cells[y * app.grid.width + x];
                let symbol = if age > 0 {
                    cell_str.clone()
                } else {
                    empty_str.clone()
                };

                let style = if x == app.cursor_x && y == app.cursor_y && app.cursor_visible {
                    Style::default().bg(Color::LightGreen)
                } else {
                    Style::default().fg(age_color(age))
                };
                spans.push(Span::styled(symbol, style));
            }
            let row_line = Line::from(spans);
            for _ in 0..zoom {
                lines.push(row_line.clone());
            }
        }
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
        let cell_state = if app.grid.cells[app.cursor_y * app.grid.width + app.cursor_x] > 0 {
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
        " [spc] pause  [n] step  [r] rand  [tab] cursor  [±] speed  [\\[\\]] zoom  [c] clear  [q] quit ",
        dim,
    ));

    let status_line = Line::from(spans);
    let status_widget =
        Paragraph::new(status_line).style(Style::default().bg(Color::Rgb(45, 45, 65)));
    frame.render_widget(status_widget, chunks[1]);
}
