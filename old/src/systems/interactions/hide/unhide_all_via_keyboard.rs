use specs::prelude::*;
use crate::{
  utilities::Key,
  resources::{
    InputState,
    events::{GeometryAction, GeometryActionChannel},
  },
};

pub struct UnhideAllViaKeyboard;

impl<'a> System<'a> for UnhideAllViaKeyboard {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, GeometryActionChannel>,
  );

  fn run(&mut self, (input_state, mut geometry_action_channel): Self::SystemData) {
    if input_state.keyboard.just_activated(Key::H) && input_state.keyboard.is_shift_activated() && input_state.keyboard.is_command_activated() {
      geometry_action_channel.single_write(GeometryAction::UnhideAll);
    }
  }
}