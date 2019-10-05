use specs::prelude::*;
use crate::{
  util::Vector2,
  resources::{InputState, Viewport},
};

static SCROLL_SPEED : f64 = 1.0; // Can be adjusted

pub struct ViewportSystem;

impl<'a> System<'a> for ViewportSystem {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, Viewport>,
  );

  fn run(&mut self, (
    mouse,
    mut vp
  ): Self::SystemData) {
    let Vector2 { x, y } = mouse.rel_scroll;
    if x != 0.0 && y != 0.0 {
      // TODO: Normalize to actual size
      let diff = vec2![-x, y] * /* delta.duration().as_secs_f64() */ 0.016 * SCROLL_SPEED; // TODO

      // Set the viewport
      vp.virtual_center += diff;

      // Push the event
      // events.push(Event::Viewport(ViewportEvent::Move(diff)));
    }
  }
}