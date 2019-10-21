use specs::prelude::*;
use crate::utilities::*;

pub type VirtualPoint = VirtualPosition;

impl Component for VirtualPoint {
  type Storage = VecStorage<Self>;
}