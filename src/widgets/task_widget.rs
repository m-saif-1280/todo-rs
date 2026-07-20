use crate::Task;
use ratatui::{
    macros::{horizontal, span},
    prelude::*,
    style::{Color, Style},
    widgets::{Block, Widget},
};
use std::str::Lines;

pub struct TaskWidget<'a> {
    task: &'a Task,
    is_focused: bool,
    cross_axis_size: u16,
}

impl<'a> TaskWidget<'a> {
    pub fn new(task: &'a Task, is_focused: bool, cross_axis_size: u16) -> Self {
        Self {
            task,
            is_focused,
            cross_axis_size,
        }
    }
}

impl<'a> TaskWidget<'a> {
    #[inline]
    fn task_lines<'b>(&'b self) -> Lines<'b> {
        self.task.title().lines()
    }
    pub fn calc_height(&self) -> u16 {
        self.task_lines()
            .map(|line| {
                let len = line.len() as u16;
                if len > 0 && self.cross_axis_size > 0 {
                    len.div_ceil(self.cross_axis_size)
                } else {
                    1
                }
            })
            .sum::<u16>()
            + 2 /* Needed to account for the border height */
    }

    fn calc_widths<'b>(&'b self) -> impl Iterator<Item = Constraint> + 'b {
        self.task_lines().map(|line| {
            let height = (line.len() as u16).div_ceil(self.cross_axis_size);
            Constraint::Length(height)
        })
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
            });

        let inner_area = block.inner(area);

        block.render(area, buf);

        let chunks = horizontal![==4, *=1].split(inner_area);
        let text_chunks = Layout::vertical(self.calc_widths()).split(chunks[1]);
        let text_chunks_iter = text_chunks.into_iter();

        let text_color = if self.is_focused {
            Color::White
        } else {
            Color::Gray
        };
        let text_style = if self.task.done() {
            Style::from(text_color).crossed_out()
        } else {
            Style::from(text_color)
        };

        span!(text_color; "[{}]", if self.task.done() { '#' } else { ' ' }).render(chunks[0], buf);

        self.task_lines()
            .map(|line| span!(text_style; line))
            .zip(text_chunks_iter)
            .for_each(|(span, chunk)| span.render(*chunk, buf));
    }
}
