use specs::prelude::*;
use crate::{
  util::Key,
  resources::InputState,
  systems::events::{GeometryAction, GeometryActionChannel},
};

pub struct RemoveSelectedViaDelete;

impl<'a> System<'a> for RemoveSelectedViaDelete {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, GeometryActionChannel>,
  );

  fn run(&mut self, (input_state, mut geometry_action_channel): Self::SystemData) {
    if input_state.keyboard.just_activated(Key::Backspace) || input_state.keyboard.just_activated(Key::Delete) {
      geometry_action_channel.single_write(GeometryAction::RemoveSelected);
    }
  }
}