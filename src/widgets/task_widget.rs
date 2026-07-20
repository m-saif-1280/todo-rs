use crate::Task;
use ratatui::{
    macros::{constraints, span},
    prelude::*,
    widgets::{Block, Widget},
};

pub struct TaskWidget<'a> {
    pub task: &'a Task,
}

impl<'a> Widget for TaskWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().border_type(ratatui::widgets::BorderType::Rounded);

        let inner_area = block.inner(area);

        block.render(area, buf);

        let chunks = Layout::horizontal(constraints![==4, *=1]).split(inner_area);

        span!("[{}]", if self.task.done() { '#' } else { ' ' }).render(chunks[0], buf);
        span!(self.task.title()).render(chunks[1], buf);
    }
}
