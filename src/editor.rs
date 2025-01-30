use crate::terminal::{Position, Size, Terminal};
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers,
};

use std::io::Error;

/// The `Editor` struct represents a basic text editor.
///
/// This struct manages the main editor loop, user inputs, and screen rendering.
/// The editor allows exiting using `Ctrl+C` and handles terminal mode changes
/// for a better user experience.
pub struct Editor {
    /// A flag to indicate whether the editor should quit.
    should_quit: bool,
}

#[allow(clippy::new_without_default)]
impl Editor {
    /// Creates a new `Editor` instance.
    ///
    /// The editor starts with the `should_quit` flag set to `false`.
    ///
    /// # Examples
    /// ```
    /// let editor = Editor::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('c') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;

            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
