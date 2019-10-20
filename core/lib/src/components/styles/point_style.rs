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