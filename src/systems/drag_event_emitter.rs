use specs::prelude::*;
use shrev::EventChannel;
use crate::resources::{InputState, DragEvent};

pub struct DragEventEmitter {
  is_dragging: bool,
}

impl Default for DragEventEmitter {
  fn default() -> Self {
    Self { is_dragging: false }
  }
}

impl<'a> System<'a> for DragEventEmitter {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, EventChannel<DragEvent>>,
  );

  fn run(&mut self, (input_state, mut drag_events): Self::SystemData) {
    if !self.is_dragging && input_state.mouse_left_button.just_activated() {
      self.is_dragging = true;
      drag_events.single_write(DragEvent::Begin);
    } else if self.is_dragging && input_state.mouse_left_button.just_deactivated() {
      self.is_dragging = false;
      drag_events.single_write(DragEvent::End);
    } else if self.is_dragging {
      if input_state.mouse_rel_movement.is_not_zero() {
        drag_events.single_write(DragEvent::Move(input_state.mouse_rel_movement));
      }
    }
  }
}