use specs::prelude::*;
use crate::math::*;

pub struct LineStyle {
  pub color: Color,
  pub width: f64,
}

impl Component for LineStyle {
  type Storage = VecStorage<Self>;
}