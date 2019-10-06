use specs::prelude::*;
use shrev::EventChannel;
use crate::{
    util::Vector2,
    resources::{InputState, Viewport, ViewportEvent, ToolState},
};

pub struct DragViewportSystem;

impl<'a> System<'a> for DragViewportSystem {
    type SystemData = (
        Read<'a, InputState>,
        Read<'a, ToolState>,
        Write<'a, EventChannel<ViewportEvent>>,
        Write<'a, Viewport>,
    );

    fn run(&mut self, (input_state, tool_state, mut vp_events, mut vp): Self::SystemData) {
        match *tool_state {
            ToolState::ViewportDrag => {
                if !input_state.mouse_left_button.is_pressed() {
                    return;
                }
                if input_state.mouse_pressed_start_point == None {
                    return;
                }
                // NOTE: diff does not commit incrementally.
                let diff = input_state.mouse_abs_pos - input_state.mouse_pressed_start_point.unwrap();
                let diff_virtual = vec2![- diff.x * vp.virtual_width() / vp.actual_width(), diff.y * vp.virtual_height() / vp.actual_height()];
                vp.virtual_center = vp.virtual_previous_center.unwrap() + diff_virtual;
                // Push the event
                //vp_events.single_write(ViewportEvent::Move(diff));
            },
            _ => ()
        }

    }
}