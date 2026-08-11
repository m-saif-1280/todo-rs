use ratatui::{
    crossterm::event::{Event, KeyCode},
    macros::{horizontal, vertical},
    prelude::{Buffer, Rect, StatefulWidget},
    widgets::{Block, Clear, Paragraph, Widget},
};
use tui_input::{Input, backend::crossterm::EventHandler};

use super::{AppAction, HandleEvent, TaskAction};
use crate::Task;

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
}

impl HandleEvent for AddTaskScreenState {
    fn handle_event(&mut self, event: &Event) -> AppAction {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Enter => {
                    let action = AppAction::Task(TaskAction::Create(Task::new(
                        self.title_input_state.value(),
                    )));
                    self.title_input_state.reset();
                    action
                }
                KeyCode::Esc => {
                    self.title_input_state.reset();
                    AppAction::ToMain
                }
                _ => {
                    self.title_input_state.handle_event(event);
                    AppAction::None
                }
            }
        } else {
            AppAction::None
        }
    }
}
