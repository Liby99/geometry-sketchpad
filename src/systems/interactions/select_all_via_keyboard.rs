use specs::prelude::*;
use crate::{
  util::Key,
  resources::InputState,
  systems::events::{GeometryAction, GeometryActionChannel},
};

pub struct SelectAllViaKeyboard;

impl<'a> System<'a> for SelectAllViaKeyboard {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, GeometryActionChannel>,
  );

  fn run(&mut self, (input_state, mut geometry_action_channel): Self::SystemData) {
    if input_state.keyboard.is_activated(Key::LCommand) || input_state.keyboard.is_activated(Key::RCommand) {
      if input_state.keyboard.just_activated(Key::A) {
        geometry_action_channel.single_write(GeometryAction::SelectAll);
      } else if input_state.keyboard.just_activated(Key::D) {
        geometry_action_channel.single_write(GeometryAction::DeselectAll);
      }
    }
  }
}