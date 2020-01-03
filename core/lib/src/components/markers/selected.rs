use specs::prelude::*;

#[derive(Default, Debug, Copy, Clone)]
pub struct Selected;

impl Component for Selected {
  type Storage = NullStorage<Self>;
}
