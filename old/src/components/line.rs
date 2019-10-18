use specs::prelude::*;
use crate::utilities::Color;
pub use crate::utilities::Line;

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
  Ray(Entity, Entity), // Ray from one point toward another
  Segment(Entity, Entity), // Segment from one point to another
  Parallel(Entity, Entity), // (line_entity, point_entity)
  Perpendicular(Entity, Entity), // (line_entity, point_entity)
}

impl Component for SymbolicLine {
  type Storage = VecStorage<Self>;
}

impl Component for Line {
  type Storage = VecStorage<Self>;
}