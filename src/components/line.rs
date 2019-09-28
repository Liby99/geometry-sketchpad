use specs::prelude::*;
use crate::util::Color;
pub use crate::math::Line;

#[derive(Debug, Copy, Clone)]
pub struct LineStyle {
  pub width: f64,
  pub color: Color,
}

impl Component for LineStyle {
  type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone)]
pub enum SymbolicLine {
  TwoPoints(Entity, Entity), // Should be two points
  Parallel(Entity, Entity), // (line_entity, point_entity)
}

impl Component for SymbolicLine {
  type Storage = VecStorage<Self>;
}

impl Component for Line {
  type Storage = VecStorage<Self>;
}