use specs::prelude::*;
use crate::{
  resources::{InputState, SnapPoint, Viewport, ViewportTransform},
  components::{Point, Line},
};

pub struct SnapPointSystem;

impl<'a> System<'a> for SnapPointSystem {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, Viewport>,
    Write<'a, SnapPoint>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, Line>,
  );

  fn run(&mut self, (
    input_state,
    viewport,
    mut snap_point,
    points,
    lines,
  ): Self::SystemData) {
    let mouse_pos = input_state.mouse_abs_pos;
    let virtual_mouse_pos = input_state.mouse_abs_pos.to_virtual(&*viewport);

  }
}