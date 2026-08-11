use ratatui::{
    prelude::{Buffer, Rect, StatefulWidget},
    text::Line,
    widgets::Block,
};
use tui_widget_list::{ListBuilder, ListState, ListView};

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
