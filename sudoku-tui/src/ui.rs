use tui::{
    backend::Backend,
    widgets::{Block, Borders, Table, Row, Cell, Paragraph},
    layout::{Constraint, Layout, Direction},
    Frame, style::{Style, Modifier, Color}
};

use crate::{app_state::{App, CellValue}, counter::CounterUpTo};

/// Build the cell content at a given position.
fn cell_at(app: &App, row: usize, col: usize) -> Cell<'static> {
    let mut style = Style::default();
    let text = match app.value_at(row, col) {
        CellValue::Pinned(v) => {
            style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
            format!(" {v} ")
        },
        CellValue::Solution(v) => {
            if app.all_vals_at(row, col).len() == 1 {
                style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
            }
            format!(" {v} ")
        }
        CellValue::NoSolution => {
            " . ".to_owned()
        }
    };
    if (row, col) == app.current_pos() {
        style = style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
    }
    Cell::from(text).style(style)
}

/// Define the UI for a given state of the application.
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(14),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(f.size());

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
    .block(Block::default().title("Sudoku").borders(Borders::ALL))
    .widths(&widths);
	f.render_widget(table, chunks[0]);

    let (row, col) = app.current_pos();
    let all_sols_par = Paragraph::new(
        match app.value_at(row, col) {
            CellValue::Pinned(v) => format!("Cell pinned to {v}."),
            CellValue::Solution(v) => {
                let values = app.all_vals_at(row, col);
                if values.len() == 1 {
                    format!("Cell has to be {v}.")
                } else {
                    format!("Allowed values: {:?}.", values)
                }
            },
            CellValue::NoSolution => "No solution.".to_owned(),
        }
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(all_sols_par, chunks[1]);

    let n_sols_par = Paragraph::new(
        match app.n_solutions() {
            CounterUpTo::Exactly(n) =>
                format!("This puzzle has {n} solutions."),
            CounterUpTo::MoreThan(n) =>
                format!("This puzzle has more than {n} solutions."),
        }
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(n_sols_par, chunks[2]);
}
