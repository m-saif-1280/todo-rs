use crate::Task;
use ratatui::{
    macros::{horizontal, span},
    prelude::*,
    style::Color,
    widgets::{Block, Widget},
};

pub struct TaskWidget<'a> {
    pub task: &'a Task,
    pub is_focused: bool,
}

impl<'a> TaskWidget<'a> {
    pub fn new(task: &'a Task, is_focused: bool) -> Self {
        Self { task, is_focused }
    }
}

impl<'a> Widget for TaskWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(if self.is_focused {
                Color::White
            } else {
                Color::DarkGray
            })
            .style(if self.is_focused {
                Color::White
            } else {
                Color::Gray
            });

        let inner_area = block.inner(area);

        block.render(area, buf);

        let chunks = horizontal![==4, *=1].split(inner_area);

        span!("[{}]", if self.task.done() { '#' } else { ' ' }).render(chunks[0], buf);
        span!(self.task.title()).render(chunks[1], buf);
    }
}
