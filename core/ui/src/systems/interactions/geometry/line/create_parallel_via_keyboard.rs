use crate::resources::*;
use core_lib::events::*;
use specs::prelude::*;

#[derive(Default)]
pub struct CreateParallelViaKeyboard;

impl<'a> System<'a> for CreateParallelViaKeyboard {
  type SystemData = (Read<'a, InputState>, Write<'a, CommandEventChannel>);

  fn run(&mut self, (input_state, mut command_event_channel): Self::SystemData) {
    let cmd = input_state.keyboard.is_command_activated();
    let shift = input_state.keyboard.is_shift_activated();
    let minus = input_state.keyboard.just_activated(Key::Minus);
    if cmd && shift && minus {
      command_event_channel.single_write(CommandEvent::LineInsert(InsertLineEvent::InsertParallelFromSelection));
    }
  }
}
