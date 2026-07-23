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

    pub fn with_done(mut self, done: bool) -> Self {
        self.done = done;
        self
    }
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = String::from(title);
        self
    }

    pub fn toggle_done(&mut self) {
        self.done = !self.done
    }
}
