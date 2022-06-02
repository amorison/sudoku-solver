use tui::{
    backend::Backend,
    widgets::{Block, Borders, Table, Row, Cell, Paragraph},
    layout::{Constraint, Layout, Direction, Margin, Rect},
    Frame, style::{Style, Modifier, Color}, text::{Spans, Span}
};

use crate::{app_state::{App, CellValue}, counter::CounterUpTo};

enum CellKind {
    Current,
    Pinned,
    UniqueSol,
}

/// All the styles.
fn get_style(kind: CellKind) -> Style {
    let stl = Style::default();
    match kind {
        CellKind::Current =>
            stl.add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        CellKind::Pinned =>
            stl.fg(Color::Green).add_modifier(Modifier::BOLD),
        CellKind::UniqueSol =>
            stl.fg(Color::Cyan).add_modifier(Modifier::BOLD),
    }
}

/// Build the cell content at a given position.
fn cell_at(app: &App, row: usize, col: usize) -> Cell<'static> {
    let mut style = Style::default();
    let text = match app.value_at(row, col) {
        CellValue::Pinned(v) => {
            style = style.patch(get_style(CellKind::Pinned));
            format!(" {v} ")
        },
        CellValue::Solution(v) => {
            if app.all_vals_at(row, col).len() == 1 {
                style = style.patch(get_style(CellKind::UniqueSol));
            }
            format!(" {v} ")
        }
        CellValue::NoSolution => {
            " . ".to_owned()
        }
    };
    if (row, col) == app.current_pos() {
        style = style.patch(get_style(CellKind::Current));
    }
    Cell::from(text).style(style)
}

/// Define the UI for a given state of the application.
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let height = 19;
    let width = 73;
    let full_area = f.size();
    if full_area.width < width || full_area.height < height {
        f.render_widget(
            Paragraph::new(vec![
                Spans::from(format!("Need {width}x{height} window")),
                Spans::from("q to quit"),
            ]),
            full_area
        );
        return;
    }
    let area = Rect {
        x: (full_area.width - width) / 2,
        y: (full_area.height - height) / 2,
        height,
        width
    };
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(43),
            Constraint::Length(1),
            Constraint::Length(29),
        ])
        .split(area);
    let help_chunks = chunks[2];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(13),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(chunks[0]);
    let help_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(13),
            Constraint::Percentage(6),
        ])
        .split(help_chunks);

    f.render_widget(
        Block::default().title("Sudoku").borders(Borders::ALL),
        chunks[0]
    );
    let mut widths = [Constraint::Length(3); 11];
    widths[3] = Constraint::Length(1);
    widths[7] = Constraint::Length(1);
    let table = Table::new(
        (0..11).map(|ir| {
            let row = ir - ir / 4;
            if ir == 3 || ir == 7 {
                Row::default().height(1)
            } else {
                Row::new(
                    (0..11).map(|ic| {
                        let col = ic - ic / 4;
                        if ic == 3 || ic == 7 {
                            Cell::default()
                        } else {
                            cell_at(app, row, col)
                        }
                    })
                )
            }
        })
    )
    .widths(&widths);
	f.render_widget(table, chunks[0].inner(&Margin {vertical: 1, horizontal: 2}));

    let (row, col) = app.current_pos();
    let all_sols_par = Paragraph::new(
        match app.value_at(row, col) {
            CellValue::Pinned(v) => format!("Cell pinned to {v}"),
            CellValue::Solution(v) => {
                let values = app.all_vals_at(row, col);
                if values.len() == 1 {
                    format!("Cell has to be {v}")
                } else {
                    format!("Could be: {:?}", values)
                }
            },
            CellValue::NoSolution => "No solution.".to_owned(),
        }
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(all_sols_par, chunks[1]);

    let n_sols_par = Paragraph::new(
        match app.n_solutions() {
            CounterUpTo::Exactly(0) =>
                "No solution".to_owned(),
            CounterUpTo::Exactly(1) =>
                "Unique solution".to_owned(),
            CounterUpTo::Exactly(n) =>
                format!("{n} solutions"),
            CounterUpTo::MoreThan(n) =>
                format!("More than {n} solutions"),
        }
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(n_sols_par, chunks[2]);

    let bold = Style::default().add_modifier(Modifier::BOLD);
    let cmd_text = vec![
        Spans::from(vec![
            Span::styled("<hjkl, wsad>", bold), Span::raw(": move"),
        ]),
        Spans::from(vec![
            Span::styled("<HJKL, WSAD>", bold), Span::raw(": block-move"),
        ]),
        Spans::from(vec![
            Span::styled("<1-9>", bold), Span::raw(": pin value"),
        ]),
        Spans::from(vec![
            Span::styled("<0, Space, Suppr>", bold), Span::raw(": unpin"),
        ]),
        Spans::from(vec![
            Span::styled("<c>", bold), Span::raw(": clear"),
        ]),
        Spans::from(vec![
            Span::styled("<q>", bold), Span::raw(": quit"),
        ]),
    ];
    let cmd_par = Paragraph::new(cmd_text)
        .block(Block::default().borders(Borders::ALL).title("Commands"));
    f.render_widget(cmd_par, help_chunks[0]);

    let lgd_text = vec![
        Spans::from(vec![
            Span::styled(" 1 ", get_style(CellKind::Current)),
            Span::raw(": current cell")
        ]),
        Spans::from(vec![
            Span::styled(" 2 ", get_style(CellKind::Pinned)),
            Span::raw(": pinned value")
        ]),
        Spans::from(vec![
            Span::styled(" 3 ", get_style(CellKind::UniqueSol)),
            Span::raw(": only one possibility")
        ]),
        Spans::from(vec![
            Span::raw(" 4 : several possibilities")
        ]),
    ];
    let lgd_par = Paragraph::new(lgd_text)
        .block(Block::default().borders(Borders::ALL).title("Legend"));
    f.render_widget(lgd_par, help_chunks[1]);
}
