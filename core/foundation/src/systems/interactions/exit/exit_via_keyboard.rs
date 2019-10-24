use specs::prelude::*;
use crate::{resources::*, events::*};

#[derive(Default)]
pub struct ExitViaKeyboard;

impl<'a> System<'a> for ExitViaKeyboard {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, ExitEventChannel>,
  );

  fn run(&mut self, (
    input_state,
    mut exit_event_channel,
  ): Self::SystemData) {
    // println!("exit via keyboard: Command {}, Q {}, W {}", input_state.keyboard.is_command_activated(), input_state.keyboard.is_activated(Key::Q), input_state.keyboard.is_activated(Key::W));
    if input_state.keyboard.is_command_activated() && (input_state.keyboard.just_activated(Key::Q) || input_state.keyboard.just_activated(Key::W)) {
      exit_event_channel.single_write(ExitEvent);
    }
  }
}