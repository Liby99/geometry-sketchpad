use specs::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum SymbolicCircle {
    CenterRadius(Entity, Entity),
}

impl Component for SymbolicCircle {
    type Storage = VecStorage<Self>;
}
