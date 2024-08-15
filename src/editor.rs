use std::io::Error;

use crossterm::event::Event;
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};

use crate::terminal_control;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        terminal_control::initialize().unwrap();
        self.refresh_screen().unwrap();
        let result = self.repl();
        terminal_control::terminate().unwrap();
        result.unwrap();
        
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        while !self.should_quit {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event : &Event) {
        if let Key(KeyEvent {
            code, 
            modifiers, 
            ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => {}
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Self::draw_lines();
        Ok(())
    }

    fn draw_lines() {
        let size = terminal_control::terminal_size();
        for i in 0..size.1 {
            terminal_control::write_at_position(0, i, "~").unwrap();
        }
    }

}