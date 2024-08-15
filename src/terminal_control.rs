use std::io::{stdout, Error};

use crossterm::{cursor::MoveTo, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};

pub fn initialize() -> Result<(), Error> {
    enable_raw_mode()?;
    clear_screen()
}

pub fn terminate() -> Result<(), Error> {
    clear_screen()?;
    print!("Goodbye...\r\n");
    disable_raw_mode()
}

pub fn write_at_position(column: u16, row: u16, text: &str) -> Result<(), std::io::Error> {
    execute!(stdout(), MoveTo(column, row))?;
    print!("{text}");
    Ok(())
}

pub fn clear_screen() -> Result<(), std::io::Error> {
    execute!(stdout(), Clear(ClearType::All))
}

pub fn terminal_size() -> (u16, u16) {
    crossterm::terminal::size().unwrap()
}