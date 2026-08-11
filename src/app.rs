use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::macros::{span, vertical};
use ratatui::{DefaultTerminal, crossterm};

use crate::Task;
use crate::TaskStore;
use crate::screens::{AddTaskScreen, AddTaskScreenState};
use crate::screens::{TaskScreen, TaskScreenState};

pub struct App {
    terminal: DefaultTerminal,
    is_running: bool,
    tasks: Vec<Task>,
    tasklist_state: TaskScreenState,
    is_adding_task: bool,
    adding_task_state: AddTaskScreenState,
    save_indicator: &'static str,
}

impl App {
    pub fn new() -> Self {
        Self {
            adding_task_state: AddTaskScreenState::default(),
            terminal: ratatui::init(),
            is_adding_task: false,
            is_running: true,
            tasks: Vec::new(),
            tasklist_state: TaskScreenState::default(),
            save_indicator: "",
        }
    }
}

impl App {
    pub fn load_tasks(&mut self) -> std::io::Result<()> {
        self.tasks = TaskStore::load().or_else(|err| {
            if let std::io::ErrorKind::NotFound = err.kind() {
                Ok(Vec::new())
            } else {
                Err(err)
            }
        })?;
        Ok(())
    }
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    pub fn draw(&mut self) {
        let _ = self.terminal.draw(|frame| {
            let master_chunks = vertical![*=1, ==1].split(frame.area());
            frame.render_stateful_widget(
                TaskScreen { tasks: &self.tasks },
                master_chunks[0],
                &mut self.tasklist_state,
            );
            frame.render_widget(span!(self.save_indicator), master_chunks[1]);

            if self.is_adding_task {
                frame.render_stateful_widget(
                    AddTaskScreen,
                    frame.area(),
                    &mut self.adding_task_state,
                );
            }
        });
    }

    pub fn handle_event(&mut self) -> std::io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            let event = event::read()?;
            self.save_indicator = "";

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
                            self.save_tasks()?;
                        }
                        KeyCode::Esc => {
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
                            if let Some(idx) = self.tasklist_state.selected() {
                                self.tasks[idx].toggle_done();
                            }
                            self.save_tasks()?;
                        }
                        KeyCode::Delete => {
                            if let Some(idx) = self.tasklist_state.selected()
                                && idx < self.tasks.len()
                            {
                                self.tasks.remove(idx);
                                self.tasklist_state.deselect();
                            }
                            self.save_tasks()?;
                        }
                        KeyCode::Char('a') => {
                            self.is_adding_task = true;
                        }
                        KeyCode::Char('s') => self.save_tasks()?,
                        KeyCode::Char('p') if self.tasklist_state.selected().is_some() => {
                            let idx = self.tasklist_state.selected().unwrap();
                            let task = self.tasks.remove(idx);
                            let new_priority = task.priority().next();
                            self.tasks.insert(idx, task.with_priority(new_priority));
                            self.save_tasks()?;
                        }
                        _ => {}
                    }
                }
            }
        };

        Ok(())
    }
    #[inline]
    pub fn save_tasks(&mut self) -> std::io::Result<()> {
        self.save_indicator = "Saved!";
        TaskStore::save(&self.tasks)
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
