use specs::prelude::*;
use crate::math::*;

pub struct PointStyle {
  pub color: Color,
  pub radius: f64,
  pub border_color: Color,
  pub border_width: f64,
}

impl Default for PointStyle {
  fn default() -> Self {
    Self {
      color: Color::red(),
      radius: 3.0,
      border_color: Color::black(),
      border_width: 1.5,
    }
  }
}

impl Component for PointStyle {
  type Storage = VecStorage<Self>;
}