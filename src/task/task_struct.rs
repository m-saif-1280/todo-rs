use super::Priority;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Task {
    title: String,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    priority: Priority,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            done: false,
            title: String::from(title),
            priority: Priority::default(),
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
    pub fn priority(&self) -> Priority {
        self.priority
    }

    pub fn with_done(mut self, done: bool) -> Self {
        self.done = done;
        self
    }
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = String::from(title);
        self
    }
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn toggle_done(&mut self) {
        self.done = !self.done
    }
}
