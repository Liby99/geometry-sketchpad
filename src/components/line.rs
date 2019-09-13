use specs::prelude::*;
use crate::util::Color;
use crate::math::Vector2;

pub struct LineStyle {
  pub width: f64,
  pub color: Color,
}

impl Component for LineStyle {
  type Storage = VecStorage<Self>;
}

pub enum SymbolicLine {
  TwoPoints(Entity, Entity), // Should be two points
}

impl Component for SymbolicLine {
  type Storage = VecStorage<Self>;
}

pub struct Line {
  pub origin: Vector2,
  pub direction: Vector2,
}

impl Component for Line {
  type Storage = VecStorage<Self>;
}