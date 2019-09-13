use specs::prelude::*;
use crate::math::Vector2;

pub struct PointStyle {
  pub radius: f64,
  pub color: [f32; 4],
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

pub struct Point(pub Option<Vector2>);

impl Component for Point {
  type Storage = VecStorage<Self>;
}