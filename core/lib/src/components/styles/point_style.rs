use specs::prelude::*;
use crate::math::*;

#[derive(Debug, Copy, Clone)]
pub struct PointStyle {
  pub color: Color,
  pub radius: f64,
  pub border_color: Color,
  pub border_width: f64,
}

impl Component for PointStyle {
  type Storage = VecStorage<Self>;
}

impl PointStyle {
  pub fn apply_alpha(self, a: f32) -> Self {
    Self {
      color: self.color.apply_alpha(a),
      radius: self.radius,
      border_color: self.border_color.apply_alpha(a),
      border_width: self.border_width,
    }
  }

  pub fn resize(self, dr: f64) -> Self {
    Self {
      color: self.color,
      radius: self.radius + dr,
      border_color: self.border_color,
      border_width: self.border_width,
    }
  }
}