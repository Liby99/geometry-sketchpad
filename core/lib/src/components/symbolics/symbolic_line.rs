use specs::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum SymbolicLine {
  Straight(Entity, Entity), // (Point Entity, Point Entity)
  Ray(Entity, Entity), // (Point Entity, Point Entity)
  Segment(Entity, Entity), // (Point Entity, Point Entity)
  Parallel(Entity, Entity), // (Line Entity, Point Entity)
  Perpendicular(Entity, Entity), // (Line Entity, Point Entity)
}

impl Component for SymbolicLine {
  type Storage = VecStorage<Self>;
}