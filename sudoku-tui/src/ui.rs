use tui::{
    backend::Backend,
    widgets::{Block, Borders, Table, Row, Cell},
    layout::Constraint,
    Frame, style::{Style, Modifier, Color}
};

use crate::app_state::{App, CellValue};

/// Build the cell content at a given position.
fn cell_at(app: &App, row: usize, col: usize) -> Cell<'static> {
    let mut style = Style::default();
    let text = match app.value_at(row, col) {
        CellValue::Pinned(v) => {
            style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
            format!(" {v} ")
        },
        CellValue::Solution(v) => {
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
	let size = f.size();
    let table = Table::new(
        (0..9).map(|ir| Row::new(
            (0..9).map(|ic| cell_at(app, ir, ic))
        ))
    )
    .block(Block::default().title("Sudoku").borders(Borders::ALL))
    .widths(&[Constraint::Length(3); 9]);
	f.render_widget(table, size);
}
