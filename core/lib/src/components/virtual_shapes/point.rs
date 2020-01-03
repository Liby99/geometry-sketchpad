use crate::utilities::*;
use specs::prelude::*;

pub type VirtualPoint = VirtualPosition;

impl Component for VirtualPoint {
    type Storage = VecStorage<Self>;
}
