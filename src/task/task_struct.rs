use std::fmt::Display;

use super::Priority;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Default, Serialize, Deserialize)]
pub struct Task {
    title: String,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    priority: Priority,
    #[serde(default)]
    due_date: Option<OffsetDateTime>,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            done: false,
            title: String::from(title),
            priority: Priority::default(),
            due_date: Some(OffsetDateTime::now_utc()),
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
    pub fn due_date(&self) -> Option<OffsetDateTime> {
        self.due_date
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
    pub fn with_due_date(mut self, due_date: Option<OffsetDateTime>) -> Self {
        self.due_date = due_date;
        self
    }

    pub fn toggle_done(&mut self) {
        self.done = !self.done
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nPriority: {}{}",
            self.title,
            self.priority,
            self.due_date
                .map(|date| {
                    let fmt = time::format_description::parse_owned::<3>(
                        "[weekday], [month repr:long] [day], [year] at [hour repr:12]:[minute] [period]",
                    )
                    .unwrap();
                    format!("\nDue by: {}", date.format(&fmt).unwrap())
                })
                .unwrap_or_default()
        )
    }
}
