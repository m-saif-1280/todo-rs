use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::HorizontalAlignment,
    macros::{horizontal, line, vertical},
    prelude::{Buffer, Rect, StatefulWidget},
    widgets::{Block, BorderType, Clear, Padding, Paragraph, Widget},
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
        let master_chunk = horizontal!(==10%, ==80%, ==10%).split(frame_area)[1];
        let master_chunk = vertical!(==10%, ==80%, ==10%).split(master_chunk)[1];
        let master_block = Block::bordered()
            .title_top(" Enter task title ")
            .title_alignment(HorizontalAlignment::Center)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);
        let master_area = master_block.inner(master_chunk);

        Clear.render(master_chunk, buf);
        master_block.render(master_chunk, buf);

        let [input_label_chunk, input_block_chunk] = *horizontal!(==25%, *=1).split(master_area)
        else {
            return;
        };
        let input_block = Block::bordered();
        let area = input_block.inner(input_block_chunk);

        let width = area.width as usize;
        let scroll_width = state.title_input_state.visual_scroll(width) as u16;

        let widget = Paragraph::new(state.value())
            .scroll((0, scroll_width))
            .block(input_block);

        line!("Title: ").centered().render(input_label_chunk, buf);
        widget.render(input_block_chunk, buf);
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
