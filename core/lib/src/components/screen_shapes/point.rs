use specs::prelude::*;
use crate::utilities::*;

pub type ScreenPoint = ScreenPosition;

impl Component for ScreenPoint {
  type Storage = VecStorage<Self>;
}