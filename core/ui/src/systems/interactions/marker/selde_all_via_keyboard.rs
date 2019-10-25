use specs::prelude::*;
use core_lib::events::*;
use crate::resources::*;

#[derive(Default)]
pub struct SeldeAllViaKeyboard;

impl<'a> System<'a> for SeldeAllViaKeyboard {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, CommandEventChannel>,
  );

  fn run(&mut self, (input_state, mut command_event_channel): Self::SystemData) {
    if input_state.keyboard.is_command_activated() {
      let maybe_selde_event = if input_state.keyboard.just_activated(Key::A) {
        Some(SelectEvent::SelectAll)
      } else if input_state.keyboard.just_activated(Key::D) {
        Some(SelectEvent::DeselectAll)
      } else {
        None
      };
      if let Some(selde_event) = maybe_selde_event {
        command_event_channel.single_write(CommandEvent::Select(selde_event));
      }
    }
  }
}