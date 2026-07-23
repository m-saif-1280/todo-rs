use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, crossterm};
use tui_input::{Input, backend::crossterm::EventHandler};
use tui_widget_list::{ListBuilder, ListState, ListView};

use crate::Task;
use crate::widgets::TaskWidget;

pub struct App {
    terminal: DefaultTerminal,
    is_running: bool,
    tasks: Vec<Task>,
    tasklist_state: ListState,
    is_adding_task: bool,
    adding_task_state: Input,
}

impl App {
    pub fn new() -> Self {
        Self {
            adding_task_state: Input::default(),
            terminal: ratatui::init(),
            is_adding_task: false,
            is_running: true,
            tasks: (1..=10)
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
            if self.is_adding_task {
                let scroll_width =
                    self.adding_task_state
                        .visual_scroll(frame.area().width as usize) as u16;
                let widget = Paragraph::new(self.adding_task_state.value())
                    .scroll((0, scroll_width))
                    .block(Block::bordered().title_top(" Type something "));

                frame.render_widget(widget, frame.area());
            } else {
                let tasklist_builder = ListBuilder::new(|context| {
                    let task = &self.tasks[context.index];
                    let task_widget = TaskWidget::new(task, context.cross_axis_size)
                        .set_focus(context.is_selected);
                    let height = task_widget.calc_height();

                    (task_widget, height)
                });
                let list_view = ListView::new(tasklist_builder, self.tasks.len())
                    .block(Block::bordered().title_top(Line::from(" Your tasks ").centered()));
                frame.render_stateful_widget(list_view, frame.area(), &mut self.tasklist_state);
            }
        });
    }

    pub fn handle_event(&mut self) -> std::io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                if let KeyCode::Char('c') = key.code
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    self.is_running = false;
                    return Ok(());
                }

                if self.is_adding_task {
                    match key.code {
                        KeyCode::Enter => {
                            self.tasks.push(Task::new(self.adding_task_state.value()));
                            self.is_adding_task = false;
                            self.adding_task_state.reset();
                        }
                        _ => {
                            self.adding_task_state.handle_event(&event);
                        }
                    }
                } else {
                    match key.code {
                        KeyCode::Tab => self.tasklist_state.next(),
                        KeyCode::BackTab => self.tasklist_state.previous(),
                        KeyCode::Char(' ') => {
                            if let Some(idx) = self.tasklist_state.selected {
                                self.tasks[idx].toggle_done();
                            }
                        }
                        KeyCode::Delete => {
                            if let Some(idx) = self.tasklist_state.selected
                                && idx < self.tasks.len()
                            {
                                self.tasks.remove(idx);
                                self.tasklist_state.selected =
                                    self.tasklist_state.selected.map(|i| i.saturating_sub(1));
                            }
                        }
                        KeyCode::Char('a') => self.is_adding_task = true,
                        _ => {}
                    }
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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
