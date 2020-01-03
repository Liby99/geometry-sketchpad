use crate::math::*;
use specs::prelude::*;

pub type ScreenRectangle = AABB;

impl Component for ScreenRectangle {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
