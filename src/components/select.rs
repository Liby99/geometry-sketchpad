use specs::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Selected;

impl Component for Selected {
  type Storage = NullStorage<Self>;
}