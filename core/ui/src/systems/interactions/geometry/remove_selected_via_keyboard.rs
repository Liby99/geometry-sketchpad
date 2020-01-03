use crate::resources::*;
use core_lib::events::*;
use specs::prelude::*;

#[derive(Default)]
pub struct RemoveSelectedViaKeyboard;

impl<'a> System<'a> for RemoveSelectedViaKeyboard {
  type SystemData = (Read<'a, InputState>, Write<'a, CommandEventChannel>);

  fn run(&mut self, (input_state, mut command_event_channel): Self::SystemData) {
    let delete = input_state.keyboard.just_activated(Key::Delete);
    let backspace = input_state.keyboard.just_activated(Key::Backspace);
    if delete || backspace {
      command_event_channel.single_write(CommandEvent::Remove(RemoveEvent::RemoveSelected));
    }
  }
}
