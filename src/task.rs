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
