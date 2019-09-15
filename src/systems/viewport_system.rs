use specs::prelude::*;
use crate::{
  math::Vector2,
  resources::{InputState, DeltaTime, Viewport},
};

static SCROLL_SPEED : f64 = 1.0; // Can be adjusted

pub struct ViewportSystem;

impl<'a> System<'a> for ViewportSystem {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, DeltaTime>,
    Write<'a, Viewport>,
  );

  fn run(&mut self, (mouse, delta, mut vp): Self::SystemData) {
    let raw_scroll = Vector2::from(mouse.rel_scroll);
    vp.virtual_center += vec2![-raw_scroll.x, raw_scroll.y] * delta.0.as_secs_f64() * SCROLL_SPEED;
  }
}