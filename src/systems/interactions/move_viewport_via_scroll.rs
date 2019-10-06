use specs::prelude::*;
use crate::{
  utilities::Vector2,
  resources::{
    DeltaTime,
    InputState,
    events::{ViewportEvent, ViewportEventChannel},
  },
};

static SCROLL_SPEED : f64 = 1.0; // Can be adjusted

pub struct MoveViewportViaScroll;

impl<'a> System<'a> for MoveViewportViaScroll {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, DeltaTime>,
    Write<'a, ViewportEventChannel>,
  );

  fn run(&mut self, (input_state, delta_time, mut viewport_event_channel): Self::SystemData) {
    let Vector2 { x, y } = input_state.rel_scroll;
    if x != 0.0 && y != 0.0 {
      let diff = vec2![-x, y] * delta_time.get() * SCROLL_SPEED;
      viewport_event_channel.single_write(ViewportEvent::Move(diff));
    }
  }
}