use specs::prelude::*;
use crate::{
  math::Vector2,
  resources::{InputState, DeltaTime, Viewport, DirtyState},
};

static SCROLL_SPEED : f64 = 1.0; // Can be adjusted

pub struct ViewportSystem;

impl<'a> System<'a> for ViewportSystem {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, DeltaTime>,
    Write<'a, DirtyState>,
    Write<'a, Viewport>,
  );

  fn run(&mut self, (mouse, delta, mut dirty_state, mut vp): Self::SystemData) {
    let Vector2 { x, y } = Vector2::from(mouse.rel_scroll);
    if x != 0.0 && y != 0.0 {
      dirty_state.is_viewport_dirty = true;

      // TODO: Normalize to actual size
      vp.virtual_center += vec2![-x, y] * delta.0.as_secs_f64() * SCROLL_SPEED;
    }
  }
}