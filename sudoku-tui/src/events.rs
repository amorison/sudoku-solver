use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::{
    app_state::{App, Direction},
    ui::ui,
};

/// Main event loop of the application, this handles keyboard input.
pub fn event_loop<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if event::poll(Duration::from_millis(30))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('h' | 'a') | KeyCode::Left => app.move_pos(Direction::Left),
                    KeyCode::Char('j' | 's') | KeyCode::Down => app.move_pos(Direction::Down),
                    KeyCode::Char('k' | 'w') | KeyCode::Up => app.move_pos(Direction::Up),
                    KeyCode::Char('l' | 'd') | KeyCode::Right => app.move_pos(Direction::Right),
                    KeyCode::Char('H' | 'A') => app.move_pos(Direction::LeftBlock),
                    KeyCode::Char('J' | 'S') => app.move_pos(Direction::DownBlock),
                    KeyCode::Char('K' | 'W') => app.move_pos(Direction::UpBlock),
                    KeyCode::Char('L' | 'D') => app.move_pos(Direction::RightBlock),
                    KeyCode::Char('c') => app = App::default(),
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('0' | ' ') | KeyCode::Backspace | KeyCode::Delete => {
                        app.unpin_current()
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        app.pin_current(c.to_digit(10).unwrap() as u8)
                    }
                    _ => {}
                }
            }
        }
    }
}
