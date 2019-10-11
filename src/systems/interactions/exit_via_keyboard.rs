use specs::prelude::*;
use crate::{
  utilities::Key,
  resources::{
    InputState,
    events::{ExitEvent, ExitEventChannel},
  },
};

pub struct ExitViaKeyboard;

impl<'a> System<'a> for ExitViaKeyboard {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, ExitEventChannel>,
  );

  fn run(&mut self, (input_state, mut finish_event_channel): Self::SystemData) {
    // Command + Q or Command + W
    if (input_state.keyboard.just_activated(Key::Q) || input_state.keyboard.just_activated(Key::W)) &&
      input_state.keyboard.is_command_activated() {
      finish_event_channel.single_write(ExitEvent);
    }
  }
}