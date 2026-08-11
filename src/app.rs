use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::macros::{span, vertical};
use ratatui::{DefaultTerminal, crossterm};

use crate::Task;
use crate::TaskStore;
use crate::screens::{AddTaskScreen, AddTaskScreenState};
use crate::screens::{AppAction, HandleEvent, TaskAction};
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
                    let action: AppAction = self.adding_task_state.handle_event(&event);
                    self.handle_screen_event(action)?
                } else {
                    let action = self.tasklist_state.handle_event(&event);
                    self.handle_screen_event(action)?
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

    fn handle_screen_event(&mut self, action: AppAction) -> std::io::Result<()> {
        match action {
            AppAction::RequestAddTask => self.is_adding_task = true,
            AppAction::Task(t) => match t {
                TaskAction::Create(task) => {
                    self.tasks.push(task);
                    self.is_adding_task = false;
                    self.save_tasks()?
                }
                TaskAction::ToggleDone(idx) => {
                    self.tasks[idx].toggle_done();
                    self.save_tasks()?
                }
                TaskAction::Delete(idx) => {
                    self.tasks.remove(idx);
                    self.save_tasks()?
                }
                TaskAction::Save => self.save_tasks()?,
                TaskAction::NextPriority(idx) => {
                    let t = self.tasks.remove(idx);
                    let new_priority = t.priority().next();
                    self.tasks.insert(idx, t.with_priority(new_priority));
                    self.save_tasks()?
                }
                _ => {}
            },
            AppAction::None => {}
            AppAction::ToMain => self.is_adding_task = false,
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
