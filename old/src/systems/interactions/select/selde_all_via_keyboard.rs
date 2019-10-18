use specs::prelude::*;
use crate::{
  utilities::Key,
  resources::{
    InputState,
    events::{GeometryAction, GeometryActionChannel},
  },
};

pub struct SeldeAllViaKeyboard;

impl<'a> System<'a> for SeldeAllViaKeyboard {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, GeometryActionChannel>,
  );

  fn run(&mut self, (input_state, mut geometry_action_channel): Self::SystemData) {
    if input_state.keyboard.is_command_activated() {
      if input_state.keyboard.just_activated(Key::A) {
        geometry_action_channel.single_write(GeometryAction::SelectAll);
      } else if input_state.keyboard.just_activated(Key::D) {
        geometry_action_channel.single_write(GeometryAction::DeselectAll);
      }
    }
  }
}