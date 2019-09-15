use specs::prelude::*;
use piston_window::Key;
use crate::{
  components::selected::Selected,
  resources::{ToolState, InputState}
};

pub struct ChangeToolSystem;

impl<'a> System<'a> for ChangeToolSystem {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, ToolState>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (input, mut tool, mut selected): Self::SystemData) {
    if input.keyboard.just_activated(Key::S) {
      *tool = ToolState::Select;
    } else {
      if input.keyboard.just_activated(Key::P) {
        selected.clear();
        *tool = ToolState::Point;
      }
    }
  }
}