pub enum AppAction {
    None,
    RequestAddTask,
    Task(TaskAction),
}

pub enum TaskAction {
    Amend { idx: usize, new: crate::Task },
    Create(crate::Task),
    Delete(usize),
    Save,
    ToggleDone(usize),
}

impl AppAction {
    pub fn task(task: TaskAction) -> Self {
        Self::Task(task)
    }
}

/// A trait for handling events
///
/// This trait is meant to be put on the state variable
/// of a stateful widget
pub(super) trait HandleEvent {
    /// Take an event and handle it, returning an action for the app to do
    fn handle_event(&mut self, event: &ratatui::crossterm::event::Event) -> AppAction;
}
