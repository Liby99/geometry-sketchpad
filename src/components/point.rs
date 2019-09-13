use specs::prelude::*;
use crate::util::Color;
use crate::math::Vector2;

pub struct PointStyle {
  pub radius: f64,
  pub color: Color,
}

impl Component for PointStyle {
  type Storage = VecStorage<Self>;
}

pub enum SymbolicPoint {
  Free(Vector2),
}

impl Component for SymbolicPoint {
  type Storage = VecStorage<Self>;
}

pub struct Point(pub Vector2);

impl Component for Point {
  type Storage = VecStorage<Self>;
}