use specs::prelude::Entity;
use shrev::{EventChannel, ReaderId};
use crate::components::{SymbolicPoint, SymbolicLine, SymbolicCircle};

#[derive(Debug, Copy, Clone)]
pub enum GeometryAction {
  SelectAll,
  DeselectAll,
  DeselectAllExcept(Entity),
  RemoveSelected,
  HideSelected,
  UnhideAll,
  DrawParallelOnSelected,
  DrawPerpendicularOnSelected,
  DrawMidpointOnSelected,
  InsertPoint(SymbolicPoint),
  InsertLine(SymbolicLine),
  InsertCircle(SymbolicCircle),
}

pub type GeometryActionChannel = EventChannel<GeometryAction>;

pub type GeometryActionReader = ReaderId<GeometryAction>;