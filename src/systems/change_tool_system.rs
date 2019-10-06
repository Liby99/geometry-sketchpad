use specs::prelude::*;
use crate::{
  util::Key,
  resources::{InputState, ToolState},
};

pub struct ChangeToolSystem;

impl<'a> System<'a> for ChangeToolSystem {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, ToolState>,
  );

  fn run(&mut self, (input_state, mut tool_state): Self::SystemData) {
    if input_state.keyboard.just_activated(Key::S) {
      *tool_state = ToolState::Select;
    } else if input_state.keyboard.just_activated(Key::P) {
      *tool_state = ToolState::Point;
    } else if input_state.keyboard.just_activated(Key::L) {
      *tool_state = ToolState::Line;
    } else if input_state.keyboard.just_activated(Key::C) {
      *tool_state = ToolState::Circle;
    } else if input_state.keyboard.just_activated(Key::V) {
      *tool_state = ToolState::ViewportDrag;
    }
  }
}