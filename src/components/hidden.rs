use specs::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Hidden;

impl Component for Hidden {
  type Storage = NullStorage<Self>;
}