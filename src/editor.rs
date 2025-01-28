use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

// TODO: Add Default configs
impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    #[must_use]
    pub fn new() -> Self {
        Editor {}
    }

    /// Runs the editor loop and handles errors by panicking.
    ///
    /// This method calls the internal `repl` function to handle
    /// the main editor logic. If an error occurs during the execution
    /// of `repl`, this method will panic and print the error in a
    /// human-readable format.
    ///
    /// # Panics
    /// This function will panic if:
    /// - `repl` returns an `Err`, indicating a failure to read input or manage terminal modes.
    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        println!("Goodbye.\r");
    }

    /// Handles the main REPL (Read-Eval-Print Loop) for the editor.
    ///
    /// This function:
    /// - Enables the terminal's raw mode to handle input events directly.
    /// - Reads input events in a loop using `crossterm::event::read`.
    /// - Exits the loop when the user presses the 'q' key.
    /// - Disables raw mode when exiting.
    ///
    /// # Errors
    /// This function returns an error if:
    /// - `enable_raw_mode` or `disable_raw_mode` fails to switch the terminal's mode.
    /// - Reading an input event via `read` fails.
    ///
    /// # Examples
    /// ```no_run
    /// let editor = Editor::new();
    /// editor.run();
    /// ```
    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");

                if let Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }

        disable_raw_mode()?;

        Ok(())
    }
}
