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
}

impl SymbolicPoint {
  pub fn is_on_same_line_with(&self, other: &SymbolicPoint) -> bool {
    match self {
      SymbolicPoint::Free(_) | SymbolicPoint::MidPoint(_, _) => false,
      SymbolicPoint::OnLine(line_ent, _) => match other {
        SymbolicPoint::Free(_) | SymbolicPoint::MidPoint(_, _) => false,
        SymbolicPoint::OnLine(l1_ent, _) => line_ent == l1_ent,
        SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => line_ent == l1_ent || line_ent == l2_ent,
      },
      SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => match other {
        SymbolicPoint::Free(_) | SymbolicPoint::MidPoint(_, _) => false,
        SymbolicPoint::OnLine(line_ent, _) => l1_ent == line_ent || l2_ent == line_ent,
        SymbolicPoint::LineLineIntersect(l3_ent, l4_ent) => {
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