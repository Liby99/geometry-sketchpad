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

impl CircleStyle {
  pub fn apply_alpha(self, a: f32) -> Self {
    Self {
      fill: self.fill.apply_alpha(a),
      border: self.border.apply_alpha(a)
    }
  }
}