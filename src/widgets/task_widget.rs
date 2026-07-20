use crate::Task;
use ratatui::{
    macros::{horizontal, span},
    prelude::*,
    style::{Color, Style},
    widgets::{Block, Widget},
};

pub struct TaskWidget<'a> {
    task: &'a Task,
    is_focused: bool,
    title_lines: Vec<String>,
}

impl TaskWidget<'_> {
    const CHECKBOX_WIDTH: u16 = 4;
    const BORDER_WIDTH: u16 = 2;
}

impl<'a> TaskWidget<'a> {
    fn wrap_text(width: u16, text: &str) -> Vec<String> {
        let mut buffer = String::new();
        let mut lines: Vec<String> = Vec::new();

        for char in text.chars() {
            if buffer.len() as u16 > width || char == '\n' {
                lines.push(std::mem::take(&mut buffer));
                buffer.clear();
            }
            buffer.push(char);
        }
        if !buffer.is_empty() {
            lines.push(buffer);
        }
        lines
    }

    pub fn new(task: &'a Task, listview_width: u16) -> Self {
        let actual_width = listview_width - Self::CHECKBOX_WIDTH - Self::BORDER_WIDTH;
        Self {
            task,
            is_focused: false,
            title_lines: Self::wrap_text(actual_width, task.title()),
        }
    }

    pub fn is_focused(mut self, is_focused: bool) -> Self {
        self.is_focused = is_focused;
        self
    }
}

impl<'a> TaskWidget<'a> {
    pub fn calc_height(&self) -> u16 {
        /* Needed to account for the border height */
        self.title_lines.len() as u16 + 2
    }

    fn calc_widths(&self) -> Vec<Constraint> {
        vec![Constraint::Length(1); self.title_lines.len()]
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

        self.title_lines
            .into_iter()
            .map(|line| span!(text_style; line))
            .zip(text_chunks_iter)
            .for_each(|(span, chunk)| span.render(*chunk, buf));
    }
}

#[cfg(test)]
mod tests {
    use super::TaskWidget;

    #[test]
    fn test_long_text() {
        let text = TaskWidget::wrap_text(20, "This is some text that should be 20+ chars");
        assert_eq!(
            text,
            vec![
                String::from("This is some text th"),
                String::from("at should be 20+ cha"),
                String::from("rs")
            ]
        )
    }
}
