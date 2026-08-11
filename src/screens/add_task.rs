use ratatui::{
    crossterm::event::Event,
    macros::{horizontal, vertical},
    prelude::{Buffer, Rect, StatefulWidget},
    widgets::{Block, Clear, Paragraph, Widget},
};
use tui_input::{Input, backend::crossterm::EventHandler};

pub struct AddTaskScreen;

#[derive(Default)]
pub struct AddTaskScreenState {
    title_input_state: Input,
}

impl StatefulWidget for AddTaskScreen {
    type State = AddTaskScreenState;

    fn render(self, frame_area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunk = horizontal!(==10%, ==80%, ==10%).split(frame_area)[1];
        let chunk = vertical!(==10%, ==80%, ==10%).split(chunk)[1];
        let block = Block::bordered().title_top(" Enter task title ");
        let area = block.inner(chunk);

        let width = area.width as usize;
        let scroll_width = state.title_input_state.visual_scroll(width) as u16;

        let widget = Paragraph::new(state.value())
            .scroll((0, scroll_width))
            .block(block);

        Clear.render(chunk, buf);
        widget.render(chunk, buf);
    }
}

impl AddTaskScreenState {
    pub fn value(&self) -> &str {
        self.title_input_state.value()
    }
    pub fn reset(&mut self) {
        self.title_input_state.reset();
    }
    pub fn handle_event(&mut self, evt: &Event) {
        self.title_input_state.handle_event(evt);
    }
}
