use crate::resources::*;
use core_lib::events::*;
use specs::prelude::*;

#[derive(Default)]
pub struct UndoRedoViaKeyboard;

impl<'a> System<'a> for UndoRedoViaKeyboard {
    type SystemData = (Read<'a, InputState>, Write<'a, HistoryEventChannel>);

    fn run(&mut self, (input_state, mut history_event_channel): Self::SystemData) {
        let cmd = input_state.keyboard.is_command_activated();
        let shift = input_state.keyboard.is_shift_activated();
        let z = input_state.keyboard.just_activated(Key::Z);
        if cmd && z {
            let history_event = if shift {
                HistoryEvent::Redo
            } else {
                HistoryEvent::Undo
            };
            history_event_channel.single_write(history_event);
        }
    }
}
