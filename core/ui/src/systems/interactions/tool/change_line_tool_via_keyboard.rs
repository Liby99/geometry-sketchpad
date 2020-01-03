use crate::{events::*, resources::*};
use core_lib::math::*;
use specs::prelude::*;

#[derive(Default)]
pub struct ChangeLineToolViaKeyboard;

impl<'a> System<'a> for ChangeLineToolViaKeyboard {
    type SystemData = (
        Read<'a, InputState>,
        Read<'a, ToolState>,
        Write<'a, ToolChangeEventChannel>,
    );

    fn run(&mut self, (input_state, tool_state, mut tool_change_event_channel): Self::SystemData) {
        match tool_state.get() {
            Tool::Line(_) => {
                if input_state.keyboard.just_activated(Key::D1) {
                    tool_change_event_channel
                        .single_write(ToolChangeEvent(Tool::Line(LineType::Straight)));
                } else if input_state.keyboard.just_activated(Key::D2) {
                    tool_change_event_channel
                        .single_write(ToolChangeEvent(Tool::Line(LineType::Ray)));
                } else if input_state.keyboard.just_activated(Key::D3) {
                    tool_change_event_channel
                        .single_write(ToolChangeEvent(Tool::Line(LineType::Segment)));
                }
            }
            _ => (),
        }
    }
}
