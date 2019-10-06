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
#[allow(dead_code)]
pub enum SymbolicPoint {
  Free(Vector2),
  OnLine(Entity, f64), // Point on a line, distance t from origin
  LineLineIntersect(Entity, Entity), // Should be two entities of lines
}

impl SymbolicPoint {
  pub fn is_on_same_line_with(&self, other: &SymbolicPoint) -> bool {
    match self {
      Self::Free(_) => false,
      Self::OnLine(line_ent, _) => match other {
        Self::Free(_) => false,
        Self::OnLine(l1_ent, _) => line_ent == l1_ent,
        Self::LineLineIntersect(l1_ent, l2_ent) => line_ent == l1_ent || line_ent == l2_ent,
      },
      Self::LineLineIntersect(l1_ent, l2_ent) => match other {
        Self::Free(_) => false,
        Self::OnLine(line_ent, _) => l1_ent == line_ent || l2_ent == line_ent,
        Self::LineLineIntersect(l3_ent, l4_ent) => {
          l1_ent == l3_ent || l1_ent == l4_ent || l2_ent == l3_ent || l2_ent == l4_ent
        },
      },
    }
  }
}

impl Component for SymbolicPoint {
  type Storage = VecStorage<Self>;
}

pub type Point = Vector2;

impl Component for Point {
  type Storage = VecStorage<Self>;
}