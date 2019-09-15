use specs::prelude::*;
use crate::util::Color;
use crate::math::Vector2;

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
  OnLine(Entity, f64), // Point on a line, distance t from origin
  LineLineIntersect(Entity, Entity), // Should be two entities of lines
}

impl Component for SymbolicPoint {
  type Storage = VecStorage<Self>;
}

pub type Point = Vector2;

impl Component for Point {
  type Storage = VecStorage<Self>;
}