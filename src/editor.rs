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

    /// # Panics
    /// This function will panic if:
    /// - `enable_raw_mode` fails to enable the terminal's raw mode.
    /// - `disable_raw_mode` fails to disable the terminal's raw mode at the end.
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");

                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(err) => eprintln!("Error: {err}"),
                _ => (),
            }
        }

        disable_raw_mode().unwrap();
    }
}
