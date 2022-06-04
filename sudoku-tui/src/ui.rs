use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::{
    app_state::{App, CellValue},
    counter::CounterUpTo,
};

enum CellKind {
    Current,
    Pinned,
    UniqueSol,
}

/// All the styles.
fn get_style(kind: CellKind) -> Style {
    let stl = Style::default();
    match kind {
        CellKind::Current => stl.add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        CellKind::Pinned => stl.fg(Color::Green).add_modifier(Modifier::BOLD),
        CellKind::UniqueSol => stl.fg(Color::Cyan).add_modifier(Modifier::BOLD),
    }
}

/// Build the cell content at a given position.
fn cell_at(app: &App, row: usize, col: usize) -> Cell<'static> {
    let mut style = Style::default();
    let text = match app.value_at(row, col) {
        CellValue::Pinned(v) => {
            style = style.patch(get_style(CellKind::Pinned));
            format!(" {v} ")
        }
        CellValue::Solution(v) => {
            if app.all_vals_at(row, col).len() == 1 {
                style = style.patch(get_style(CellKind::UniqueSol));
            }
            format!(" {v} ")
        }
        CellValue::NoSolution => " . ".to_owned(),
    };
    if (row, col) == app.current_pos() {
        style = style.patch(get_style(CellKind::Current));
    }
    Cell::from(text).style(style)
}

/// Locations of the ui elements
struct UiLayout {
    /// The main sudoku grid
    grid: Rect,
    /// Set of possibilities for the current cell
    possibilities: Rect,
    /// Number of solutions of the grid
    n_solutions: Rect,
    /// List of available commands
    cmd_help: Rect,
    /// Explanations of the text styles
    legend: Rect,
}

impl UiLayout {
    /// Create the layout, centered on a given area
    fn new(area: Rect) -> Result<Self, String> {
        // grid + diagnostics, spacing, help column
        let widths: [u16; 3] = [43, 1, 29];
        // grid + commands, diagnostics + legend
        let heights: [u16; 3] = [13, 3, 3];
        let width: u16 = widths.iter().sum();
        let height: u16 = heights.iter().sum();
        if area.width < width || area.height < height {
            return Err(format!("Need {width}x{height} area"));
        }

        // the centered area of the size we need to fit all elements
        let area = Rect {
            x: (area.width - width) / 2,
            y: (area.height - height) / 2,
            height,
            width,
        };

        // split in columns
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(widths.map(Constraint::Length))
            .split(area);

        // split the right column to fit the cmd_help and legend elements
        let help_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(heights[0]),
                Constraint::Length(height - heights[0]),
            ])
            .split(chunks[2]);

        // split the left column into grid and diagnostics
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(heights.map(Constraint::Length))
            .split(chunks[0]);

        Ok(UiLayout {
            grid: chunks[0],
            possibilities: chunks[1],
            n_solutions: chunks[2],
            cmd_help: help_chunks[0],
            legend: help_chunks[1],
        })
    }
}

/// Define the UI for a given state of the application.
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let full_area = f.size();
    let layout = match UiLayout::new(full_area) {
        Ok(layout) => layout,
        Err(err) => {
            f.render_widget(
                Paragraph::new(vec![Spans::from(err), Spans::from("q to quit")]),
                full_area,
            );
            return;
        }
    };
    f.render_widget(
        Block::default().title("Sudoku").borders(Borders::ALL),
        layout.grid,
    );
    let mut widths = [Constraint::Length(3); 11];
    widths[3] = Constraint::Length(1);
    widths[7] = Constraint::Length(1);
    let table = Table::new((0..11).map(|ir| {
        let row = ir - ir / 4;
        if ir == 3 || ir == 7 {
            Row::default().height(1)
        } else {
            Row::new((0..11).map(|ic| {
                let col = ic - ic / 4;
                if ic == 3 || ic == 7 {
                    Cell::default()
                } else {
                    cell_at(app, row, col)
                }
            }))
        }
    }))
    .widths(&widths);
    f.render_widget(
        table,
        layout.grid.inner(&Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );

    let (row, col) = app.current_pos();
    let all_sols_par = Paragraph::new(match app.value_at(row, col) {
        CellValue::Pinned(v) => format!("Cell pinned to {v}"),
        CellValue::Solution(v) => {
            let values = app.all_vals_at(row, col);
            if values.len() == 1 {
                format!("Cell has to be {v}")
            } else {
                format!("Could be: {:?}", values)
            }
        }
        CellValue::NoSolution => "No solution.".to_owned(),
    })
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(all_sols_par, layout.possibilities);

    let n_sols_par = Paragraph::new(match app.n_solutions() {
        CounterUpTo::Exactly(0) => "No solution".to_owned(),
        CounterUpTo::Exactly(1) => "Unique solution".to_owned(),
        CounterUpTo::Exactly(n) => format!("{n} solutions"),
        CounterUpTo::MoreThan(n) => format!("More than {n} solutions"),
    })
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(n_sols_par, layout.n_solutions);

    let bold = Style::default().add_modifier(Modifier::BOLD);
    let cmd_text = vec![
        Spans::from(vec![
            Span::styled("<hjkl, wsad>", bold),
            Span::raw(": move"),
        ]),
        Spans::from(vec![
            Span::styled("<HJKL, WSAD>", bold),
            Span::raw(": block-move"),
        ]),
        Spans::from(vec![Span::styled("<1-9>", bold), Span::raw(": pin value")]),
        Spans::from(vec![
            Span::styled("<0, Space, Suppr>", bold),
            Span::raw(": unpin"),
        ]),
        Spans::from(vec![Span::styled("<c>", bold), Span::raw(": clear")]),
        Spans::from(vec![Span::styled("<q>", bold), Span::raw(": quit")]),
    ];
    let cmd_par =
        Paragraph::new(cmd_text).block(Block::default().borders(Borders::ALL).title("Commands"));
    f.render_widget(cmd_par, layout.cmd_help);

    let lgd_text = vec![
        Spans::from(vec![
            Span::styled(" 1 ", get_style(CellKind::Current)),
            Span::raw(": current cell"),
        ]),
        Spans::from(vec![
            Span::styled(" 2 ", get_style(CellKind::Pinned)),
            Span::raw(": pinned value"),
        ]),
        Spans::from(vec![
            Span::styled(" 3 ", get_style(CellKind::UniqueSol)),
            Span::raw(": only one possibility"),
        ]),
        Spans::from(vec![Span::raw(" 4 : several possibilities")]),
    ];
    let lgd_par =
        Paragraph::new(lgd_text).block(Block::default().borders(Borders::ALL).title("Legend"));
    f.render_widget(lgd_par, layout.legend);
}
