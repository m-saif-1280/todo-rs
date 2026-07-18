use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal, crossterm, layout::Layout, macros::constraint};

use crate::Task;
use crate::widgets::TaskWidget;

pub struct App {
    terminal: DefaultTerminal,
    is_running: bool,
    tasks: Vec<Task>,
}

impl App {
    pub fn new() -> Self {
        Self {
            terminal: ratatui::init(),
            is_running: true,
            tasks: (1..=10)
                .into_iter()
                .map(|n| {
                    let mut t = Task::new(&format!("Task #{n}"));
                    if n % 2 == 0 {
                        t.toggle_done();
                        t
                    } else {
                        t
                    }
                })
                .collect(),
        }
    }
}

impl App {
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    pub fn draw(&mut self) {
        let _ = self.terminal.draw(|frame| {
            let (task_widgets, constraints): (Vec<_>, Vec<_>) = self
                .tasks
                .iter()
                .map(|task| (task, constraint!(==3)))
                .unzip();

            let chunks = Layout::vertical(constraints).split(frame.area());
            let chunks = chunks.into_iter();

            for (task_widget, chunk) in task_widgets.into_iter().zip(chunks) {
                frame.render_widget(task_widget, *chunk);
            }
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
