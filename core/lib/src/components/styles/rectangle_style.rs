use specs::prelude::*;
use crate::math::*;
use super::LineStyle;

pub struct RectangleStyle {
  pub fill: Color,
  pub border: LineStyle,
}

impl Component for RectangleStyle {
  type Storage = VecStorage<Self>;
}