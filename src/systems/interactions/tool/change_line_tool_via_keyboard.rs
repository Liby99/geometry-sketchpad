use specs::prelude::*;
use crate::{
  utilities::Key,
  resources::{
    InputState, Tool, LineTool, ToolState,
    events::{ToolChangeEvent, ToolChangeEventChannel}
  },
};

pub struct ChangeLineToolViaKeyboard;

impl<'a> System<'a> for ChangeLineToolViaKeyboard {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, ToolState>,
    Write<'a, ToolChangeEventChannel>,
  );

  fn run(&mut self, (input_state, tool_state, mut tool_change_event_channel): Self::SystemData) {
    match tool_state.get() {
      Tool::Line(_) => {

        // First check input and line tool
        let event = if input_state.keyboard.just_activated(Key::D1) {
          Some(LineTool::Line)
        } else if input_state.keyboard.just_activated(Key::D2) {
          Some(LineTool::Ray)
        } else if input_state.keyboard.just_activated(Key::D3) {
          Some(LineTool::Segment)
        } else {
          None
        };

        // Update the line tool
        if let Some(line_tool) = event {
          tool_change_event_channel.single_write(ToolChangeEvent(Tool::Line(line_tool)));
        }
      },
      _ => (),
    }
  }
}