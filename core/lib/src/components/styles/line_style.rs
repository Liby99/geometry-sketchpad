use specs::prelude::*;
use crate::math::*;

#[derive(Debug, Copy, Clone)]
pub struct LineStyle {
  pub color: Color,
  pub width: f64,
}

impl Component for LineStyle {
  type Storage = VecStorage<Self>;
}

impl LineStyle {
  pub fn apply_alpha(self, a: f32) -> Self {
    Self {
      color: self.color.apply_alpha(a),
      width: self.width,
    }
  }
}