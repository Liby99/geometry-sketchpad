use crate::resources::*;
use core_lib::events::*;
use specs::prelude::*;

#[derive(Default)]
pub struct HideViaKeyboard;

impl<'a> System<'a> for HideViaKeyboard {
    type SystemData = (Read<'a, InputState>, Write<'a, CommandEventChannel>);

    fn run(&mut self, (input_state, mut command_event_channel): Self::SystemData) {
        if input_state.keyboard.is_command_activated() {
            if input_state.keyboard.just_activated(Key::H) {
                let hide_event = if input_state.keyboard.is_shift_activated() {
                    HideEvent::UnhideAll
                } else {
                    HideEvent::HideSelected
                };
                command_event_channel.single_write(CommandEvent::Hide(hide_event));
            }
        }
    }
}
