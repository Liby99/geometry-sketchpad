use specs::prelude::*;

#[derive(Default, Debug, Copy, Clone)]
pub struct Hidden;

impl Component for Hidden {
  type Storage = NullStorage<Self>;
}
