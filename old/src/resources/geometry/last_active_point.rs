use specs::prelude::*;
use shrev::{EventChannel, ReaderId};

pub struct LastActivePoint(Entity);

impl LastActivePoint {
  pub fn new(entity: Entity) -> Self {
    Self(entity)
  }

  pub fn get(&self) -> Entity {
    self.0
  }
}

pub type LastActivePointChannel = EventChannel<LastActivePoint>;

pub type LastActivePointReader = ReaderId<LastActivePoint>;