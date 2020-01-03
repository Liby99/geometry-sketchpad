use crate::{events::*, resources::*};
use specs::prelude::*;

#[derive(Default)]
pub struct ExitViaKeyboard;

impl<'a> System<'a> for ExitViaKeyboard {
    type SystemData = (Read<'a, InputState>, Write<'a, ExitEventChannel>);

    fn run(&mut self, (input_state, mut exit_event_channel): Self::SystemData) {
        if input_state.keyboard.is_command_activated()
            && (input_state.keyboard.just_activated(Key::Q)
                || input_state.keyboard.just_activated(Key::W))
        {
            exit_event_channel.single_write(ExitEvent);
        }
    }
}
