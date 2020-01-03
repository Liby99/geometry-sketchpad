use specs::prelude::*;

pub use crate::utilities::VirtualCircle;

impl Component for VirtualCircle {
  type Storage = VecStorage<Self>;
}
