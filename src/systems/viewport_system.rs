use specs::prelude::*;
use shrev::EventChannel;
use crate::{
  util::Vector2,
  resources::{DeltaTime, InputState, Viewport, ViewportEvent},
};

static SCROLL_SPEED : f64 = 1.0; // Can be adjusted

pub struct ViewportSystem;

impl<'a> System<'a> for ViewportSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Read<'a, InputState>,
    Write<'a, EventChannel<ViewportEvent>>,
    Write<'a, Viewport>,
  );

  fn run(&mut self, (delta_time, mouse, mut vp_events, mut vp): Self::SystemData) {
    let Vector2 { x, y } = mouse.rel_scroll;
    if x != 0.0 && y != 0.0 {
      // TODO: Normalize to actual size
      let diff = vec2![-x, y] * delta_time.get() * SCROLL_SPEED;

      // Set the viewport
      vp.virtual_center += diff;

      // Push the event
      vp_events.single_write(ViewportEvent::Move(diff));
    }
  }
}