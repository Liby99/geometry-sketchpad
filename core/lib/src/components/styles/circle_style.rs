use specs::prelude::*;
use crate::math::*;
use super::LineStyle;

#[derive(Debug, Copy, Clone)]
pub struct CircleStyle {
  pub fill: Color,
  pub border: LineStyle,
}

impl Component for CircleStyle {
  type Storage = VecStorage<Self>;
}