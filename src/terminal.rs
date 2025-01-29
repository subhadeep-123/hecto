use std::io::{stdout, Error};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), Error> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn size() -> Result<(u16, u16), Error> {
        size()
    }
}
