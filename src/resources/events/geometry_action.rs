use specs::prelude::Entity;
use shrev::{EventChannel, ReaderId};

pub enum GeometryAction {
  SelectAll,
  DeselectAll,
  DeselectAllExcept(Entity),
  RemoveSelected,
  DrawParallelOnSelected,
}

pub type GeometryActionChannel = EventChannel<GeometryAction>;

pub type GeometryActionReader = ReaderId<GeometryAction>;