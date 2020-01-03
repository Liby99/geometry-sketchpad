use specs::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Element;

impl Component for Element {
  type Storage = NullStorage<Self>;
}
