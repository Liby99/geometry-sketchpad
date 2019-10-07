use specs::prelude::*;
use crate::{
  utilities::Key,
  resources::{
    InputState,
    events::{HistoryAction, HistoryActionChannel},
  }
};

pub struct RedoViaKeyboard;

impl<'a> System<'a> for RedoViaKeyboard {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, HistoryActionChannel>,
  );

  fn run(&mut self, (input_state, mut history_action_channel): Self::SystemData) {
    if input_state.keyboard.just_activated(Key::Z) && input_state.keyboard.is_shift_activated() && input_state.keyboard.is_command_activated() {
      history_action_channel.single_write(HistoryAction::Redo);
    }
  }
}