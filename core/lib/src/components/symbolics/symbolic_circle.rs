use specs::prelude::*;

pub enum SymbolicCircle {
  CenterRadius(Entity, Entity),
}

impl Component for SymbolicCircle {
  type Storage = VecStorage<Self>;
}