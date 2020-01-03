use crate::utilities::*;
use specs::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum SymbolicPoint {
  Fixed(VirtualPosition),
  Free(VirtualPosition),
  MidPoint(Entity, Entity),                                 // (Point entity, Point entity)
  OnLine(Entity, VirtualScalar),                            // (Line entity, frac{p_to_from}{to_to_from})
  LineLineIntersect(Entity, Entity),                        // (Line entity, Line entity)
  OnCircle(Entity, f64),                                    // (Circle entity, theta)
  CircleLineIntersect(Entity, Entity, CircleIntersectId),   // (Circle entity, Line entity, Id)
  CircleCircleIntersect(Entity, Entity, CircleIntersectId), // (Circle entity, Circle entity, Id)
}

#[derive(Debug, Copy, Clone)]
pub enum CircleIntersectId {
  First,
  Second,
}

impl Component for SymbolicPoint {
  type Storage = VecStorage<Self>;
}
