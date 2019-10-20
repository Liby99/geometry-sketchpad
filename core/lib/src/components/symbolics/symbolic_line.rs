use specs::prelude::*;

pub enum SymbolicLine {
  TwoPoints(Entity, Entity), // (Point Entity, Point Entity)
  Parallel(Entity, Entity), // (Line Entity, Point Entity)
  Perpendicular(Entity, Entity), // (Line Entity, Point Entity)
}

impl Component for SymbolicLine {
  type Storage = VecStorage<Self>;
}