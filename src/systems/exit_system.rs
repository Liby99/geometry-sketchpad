use specs::prelude::*;
use crate::{
  util::Key,
  resources::{FinishState, InputState},
};

pub struct ExitSystem;

impl<'a> System<'a> for ExitSystem {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, FinishState>,
  );

  fn run(&mut self, (input_state, mut finish_state): Self::SystemData) {
    if input_state.keyboard.just_activated(Key::Q) &&
      (input_state.keyboard.is_activated(Key::LCommand) || input_state.keyboard.is_activated(Key::RCommand)) {
      finish_state.set_finished();
    }
  }
}