use specs::prelude::*;

pub struct LastActivePoint(Entity);

impl LastActivePoint {
  pub fn new(entity: Entity) -> Self {
    Self(entity)
  }

  pub fn get(&self) -> Entity {
    self.0
  }
}