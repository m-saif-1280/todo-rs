use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, crossterm};
use tui_widget_list::{ListBuilder, ListState, ListView};

use crate::Task;
use crate::widgets::TaskWidget;

pub struct App {
    terminal: DefaultTerminal,
    is_running: bool,
    tasks: Vec<Task>,
    tasklist_state: ListState,
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
            tasklist_state: ListState::default(),
        }
    }
}

impl App {
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    pub fn draw(&mut self) {
        let _ = self.terminal.draw(|frame| {
            let tasklist_builder = ListBuilder::new(|context| {
                let task = &self.tasks[context.index];

                (TaskWidget::new(task, context.is_selected), 3)
            });
            let list_view = ListView::new(tasklist_builder, self.tasks.len())
                .block(Block::bordered().title_top(Line::from(" Your tasks ").centered()));
            frame.render_stateful_widget(list_view, frame.area(), &mut self.tasklist_state);
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
                    KeyCode::Tab => self.tasklist_state.next(),
                    KeyCode::BackTab => self.tasklist_state.previous(),
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
