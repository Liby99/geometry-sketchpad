use specs::prelude::*;
use crate::{
  util::{AABB, Color},
  components::LineStyle,
};

pub type Rectangle = AABB;

impl Component for Rectangle {
  type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone)]
pub struct RectangleStyle {
  pub border: LineStyle,
  pub fill: Color,
}

impl Component for RectangleStyle {
  type Storage = VecStorage<Self>;
}