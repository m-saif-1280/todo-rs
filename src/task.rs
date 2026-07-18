use ratatui::{
    macros::{constraints, span},
    prelude::*,
    widgets::{Block, Widget},
};

pub struct Task {
    done: bool,
    title: String,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            done: false,
            title: String::from(title),
        }
    }
}

impl Task {
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn toggle_done(&mut self) {
        self.done = !self.done
    }
}

impl Widget for &Task {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().border_type(ratatui::widgets::BorderType::Rounded);

        let inner_area = block.inner(area);

        block.render(area, buf);

        let chunks = Layout::horizontal(constraints![==3, *=1]).split(inner_area);

        span!("[{}]", if self.done() { '#' } else { ' ' }).render(chunks[0], buf);
        span!(self.title()).render(chunks[1], buf);
    }
}
