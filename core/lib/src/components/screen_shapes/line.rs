use specs::prelude::*;

pub use crate::utilities::ScreenLine;

impl Component for ScreenLine {
  type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
