use crate::resources::*;
use core_lib::events::*;
use specs::prelude::*;

#[derive(Default)]
pub struct CreateMidpointViaKeyboard;

impl<'a> System<'a> for CreateMidpointViaKeyboard {
  type SystemData = (Read<'a, InputState>, Write<'a, CommandEventChannel>);

  fn run(&mut self, (input_state, mut command_event_channel): Self::SystemData) {
    let cmd = input_state.keyboard.is_command_activated();
    let no_shift = !input_state.keyboard.is_shift_activated();
    let m = input_state.keyboard.just_activated(Key::M);
    if cmd && no_shift && m {
      command_event_channel.single_write(CommandEvent::PointInsert(InsertPointEvent::InsertMidPointFromSelection));
    }
  }
}
