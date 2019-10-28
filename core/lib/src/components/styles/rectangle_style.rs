use specs::prelude::*;
use crate::math::*;
use super::LineStyle;

#[derive(Debug, Copy, Clone)]
pub struct RectangleStyle {
  pub fill: Color,
  pub border: LineStyle,
}

impl Component for RectangleStyle {
  type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}