use specs::prelude::*;
use crate::utilities::*;

pub enum SymbolicPoint {
  Fixed(VirtualPosition),
  Free(VirtualPosition),
  MidPoint(Entity, Entity), // (Point entity, Point entity)
  OnLine(Entity, f64), // (Line entity, frac{p_to_from}{to_to_from})
  LineLineIntersect(Entity, Entity), // (Line entity, Line entity)
  OnCircle(Entity, f64), // (Circle entity, theta)
  CircleLineIntersect(Entity, Entity, CircleIntersectId), // (Circle entity, Line entity, Id)
  CircleCircleIntersect(Entity, Entity, CircleIntersectId), // (Circle entity, Circle entity, Id)
}

pub enum CircleIntersectId {
  First,
  Second,
}

impl Component for SymbolicPoint {
  type Storage = VecStorage<Self>;
}