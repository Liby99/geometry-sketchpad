use specs::prelude::*;
use crate::math::*;

pub type ScreenRectangle = AABB;

impl Component for ScreenRectangle {
  type Storage = VecStorage<Self>;
}