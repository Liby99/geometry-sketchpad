use specs::prelude::*;
use geopad_core_lib::{math::*};
use crate::{resources::*, events::*};

#[derive(Default)]
pub struct ChangeToolViaKeyboard;

impl<'a> System<'a> for ChangeToolViaKeyboard {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, ToolChangeEventChannel>,
  );

  fn run(&mut self, (input_state, mut tool_change_event_channel): Self::SystemData) {
    if input_state.keyboard.just_activated(Key::S) {
      tool_change_event_channel.single_write(ToolChangeEvent(Tool::Select));
    } else if input_state.keyboard.just_activated(Key::V) {
      tool_change_event_channel.single_write(ToolChangeEvent(Tool::Viewport));
    } else if input_state.keyboard.just_activated(Key::P) {
      tool_change_event_channel.single_write(ToolChangeEvent(Tool::Point));
    } else if input_state.keyboard.just_activated(Key::L) {
      tool_change_event_channel.single_write(ToolChangeEvent(Tool::Line(LineType::Straight)));
    } else if input_state.keyboard.just_activated(Key::C) {
      tool_change_event_channel.single_write(ToolChangeEvent(Tool::Circle));
    }
  }
}