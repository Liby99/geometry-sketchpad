use specs::prelude::Entity;
use shrev::{EventChannel, ReaderId};
use crate::components::{SymbolicPoint, SymbolicLine};

pub enum GeometryAction {
  SelectAll,
  DeselectAll,
  DeselectAllExcept(Entity),
  RemoveSelected,
  DrawParallelOnSelected,
  DrawPerpendicularOnSelected,
  DrawMidpointOnSelected,
  InsertPoint(SymbolicPoint),
  InsertLine(SymbolicLine),
}

pub type GeometryActionChannel = EventChannel<GeometryAction>;

pub type GeometryActionReader = ReaderId<GeometryAction>;