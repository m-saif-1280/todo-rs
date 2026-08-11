use ratatui::{
    crossterm::event::{Event, KeyCode},
    prelude::{Buffer, Rect, StatefulWidget},
    text::Line,
    widgets::Block,
};
use tui_widget_list::{ListBuilder, ListState, ListView};

use super::{AppAction, HandleEvent, TaskAction};
use crate::Task;
use crate::widgets::TaskWidget;

pub struct TaskScreen<'a> {
    pub tasks: &'a Vec<Task>,
}

#[derive(Default)]
pub struct TaskScreenState {
    tasklist_state: ListState,
}

impl StatefulWidget for TaskScreen<'_> {
    type State = TaskScreenState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let tasklist_builder = ListBuilder::new(|context| {
            let task = &self.tasks[context.index];
            let task_widget =
                TaskWidget::new(task, context.cross_axis_size).set_focus(context.is_selected);
            let height = task_widget.calc_height();

            (task_widget, height)
        });

        let list_view = ListView::new(tasklist_builder, self.tasks.len())
            .block(Block::bordered().title_top(Line::from(" Your tasks ").centered()));
        list_view.render(area, buf, &mut state.tasklist_state);
    }
}

impl TaskScreenState {
    pub fn next(&mut self) {
        self.tasklist_state.next();
    }
    pub fn previous(&mut self) {
        self.tasklist_state.selected.map(|i| i.saturating_sub(1));
    }
    pub fn selected(&mut self) -> Option<usize> {
        self.tasklist_state.selected
    }
    pub fn deselect(&mut self) {
        self.tasklist_state.selected.take();
    }
}

impl HandleEvent for TaskScreenState {
    fn handle_event(&mut self, event: &Event) -> AppAction {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Tab => {
                    self.next();
                    AppAction::None
                }
                KeyCode::BackTab => {
                    self.previous();
                    AppAction::None
                }
                KeyCode::Char(' ') if let Some(idx) = self.selected() => {
                    AppAction::Task(TaskAction::ToggleDone(idx))
                }
                KeyCode::Delete if let Some(idx) = self.selected() => {
                    self.deselect();
                    AppAction::Task(TaskAction::Delete(idx))
                }
                KeyCode::Char('a') => AppAction::RequestAddTask,
                KeyCode::Char('s') => AppAction::Task(TaskAction::Save),
                KeyCode::Char('p') if let Some(idx) = self.selected() => {
                    AppAction::Task(TaskAction::NextPriority(idx))
                }
                _ => AppAction::None,
            }
        } else {
            AppAction::None
        }
    }
}
