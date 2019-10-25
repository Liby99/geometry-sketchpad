use specs::prelude::*;

pub use crate::utilities::ScreenCircle;

impl Component for ScreenCircle {
  type Storage = VecStorage<Self>;
}