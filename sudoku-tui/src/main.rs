use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use sudoku_tui::app_state::App;
use sudoku_tui::events::event_loop;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    // setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let app = App::default();
    let res = event_loop(&mut term, app);

    // restore terminal
    terminal::disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen)?;
    term.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
