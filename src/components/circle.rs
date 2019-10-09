use specs::prelude::*;
use crate::utilities::Color;
pub use crate::utilities::Circle;

#[derive(Debug, Copy, Clone)]
pub enum SymbolicCircle {
  CenterRadius(Entity, Entity), // Center, a point on circle
  // ThreePoints(Entity, Entity, Entity), // The three point entities
}

impl Component for SymbolicCircle {
  type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone)]
pub struct CircleStyle {
  pub width: f64,
  pub color: Color,
}

impl Component for CircleStyle {
  type Storage = VecStorage<Self>;
}

impl Component for Circle {
  type Storage = VecStorage<Self>;
}