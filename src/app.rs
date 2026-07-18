use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal, crossterm};

pub struct App {
    terminal: DefaultTerminal,
    is_running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            terminal: ratatui::init(),
            is_running: true,
        }
    }
}

impl App {
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    pub fn draw(&mut self) {
        let _ = self.terminal.draw(|frame| {
            frame.render_widget("Hello World!", frame.area());
        });
    }

    pub fn handle_event(&mut self) -> std::io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.is_running = false;
                    }
                    _ => {}
                }
            }
        };

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
