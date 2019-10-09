use specs::prelude::*;
use crate::utilities::{Color, Vector2};

#[derive(Debug, Copy, Clone)]
pub struct PointStyle {
  pub radius: f64,
  pub color: Color,
}

impl Component for PointStyle {
  type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone)]
pub enum SymbolicPoint {
  Free(Vector2),
  MidPoint(Entity, Entity), // point 1, point 2
  OnLine(Entity, f64), // Point on a line, distance t from origin
  LineLineIntersect(Entity, Entity), // Should be two entities of lines
  // OnCircle(Entity, f64), // Point on a circle, theta
  // CircleLineIntersect(Entity, Entity, u8), // circle, line, Identifier
  // CircleCircleIntersect(Entity, Entity, u8), // circle 1, circle 2, Identifier of the intersection
}

impl Component for SymbolicPoint {
  type Storage = VecStorage<Self>;
}

pub type Point = Vector2;

impl Component for Point {
  type Storage = VecStorage<Self>;
}