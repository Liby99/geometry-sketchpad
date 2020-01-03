use crate::utilities::*;
use specs::prelude::*;

pub type ScreenPoint = ScreenPosition;

impl Component for ScreenPoint {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
