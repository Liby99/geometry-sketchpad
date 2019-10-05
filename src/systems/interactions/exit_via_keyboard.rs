use specs::prelude::*;
use shrev::EventChannel;
use crate::{
  util::Key,
  systems::events::ExitEvent,
  resources::InputState,
};

pub struct ExitViaKeyboard;

impl<'a> System<'a> for ExitViaKeyboard {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, EventChannel<ExitEvent>>,
  );

  fn run(&mut self, (input_state, mut finish_event_channel): Self::SystemData) {

    // Command + Q or Command + W
    if (input_state.keyboard.just_activated(Key::Q) || input_state.keyboard.just_activated(Key::W)) &&
      (input_state.keyboard.is_activated(Key::LCommand) || input_state.keyboard.is_activated(Key::RCommand)) {
      finish_event_channel.single_write(ExitEvent);
    }
  }
}