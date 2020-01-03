use specs::prelude::*;

pub use crate::utilities::VirtualLine;

impl Component for VirtualLine {
    type Storage = VecStorage<Self>;
}
