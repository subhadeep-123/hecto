use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::stdout;

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
    pub fn new() -> Self {
        Editor { should_quit: false }
    }

    /// Runs the editor loop and handles initialization, user interaction, and termination.
    ///
    /// This method calls `initialize` to set up the terminal, enters the main
    /// REPL loop via `repl`, and finally cleans up the terminal state by calling `terminate`.
    /// Any errors during these steps will cause the program to panic.
    ///
    /// # Panics
    /// This function will panic if:
    /// - `initialize`, `repl`, or `terminate` return an error.
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    /// Initializes the editor by enabling raw mode and clearing the screen.
    ///
    /// # Errors
    /// This function will return an error if enabling raw mode or clearing the screen fails.
    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    /// Terminates the editor by disabling raw mode.
    ///
    /// # Errors
    /// This function will return an error if disabling raw mode fails.
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    /// Clears the terminal screen.
    ///
    /// This function uses `crossterm` to clear the entire terminal screen.
    ///
    /// # Errors
    /// This function will return an error if clearing the screen fails.
    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    /// Handles the main REPL (Read-Eval-Print Loop) for the editor.
    ///
    /// This method:
    /// - Processes user input using `evaluate_event`.
    /// - Updates the screen via `refresh_screen`.
    /// - Breaks the loop and exits when the `should_quit` flag is set to `true`.
    ///
    /// # Errors
    /// This function will return an error if:
    /// - Reading an input event fails.
    /// - Refreshing the screen fails.
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;

        Ok(())
    }

    /// Evaluates a user input event and updates the editor's state.
    ///
    /// If the event matches `Ctrl+C`, the `should_quit` flag is set to `true`.
    ///
    /// # Parameters
    /// - `event`: A reference to the input event to be evaluated.
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

    /// Refreshes the screen and updates the terminal display.
    ///
    /// If the `should_quit` flag is set to `true`, it clears the screen
    /// and displays a goodbye message.
    ///
    /// # Errors
    /// This function will return an error if clearing the screen fails.
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
