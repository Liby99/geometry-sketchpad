use specs::prelude::*;

pub struct LastActivePoint(pub Option<Entity>);

impl Default for LastActivePoint {
  fn default() -> Self {
    Self(None)
  }
}

impl LastActivePoint {
  pub fn set(&mut self, ent: Entity) {
    self.0 = Some(ent);
  }

  pub fn clear(&mut self) {
    self.0 = None;
  }
}