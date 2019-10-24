use specs::prelude::*;
use shrev::*;
use crate::components::{symbolics::*, styles::*};

#[derive(Debug, Clone, Copy)]
pub enum CommandEvent {

  // Insert points
  InsertPoint(SymbolicPoint),
  InsertPointWithStyle(SymbolicPoint, PointStyle),
  InsertPointByHistory(Entity, SymbolicPoint, PointStyle),

  // Insert lines
  InsertLine(SymbolicLine),
  InsertLineWithStyle(SymbolicLine, LineStyle),
  InsertLineByHistory(Entity, SymbolicLine, LineStyle),

  // Insert circles
  InsertCircle(SymbolicCircle),
  InsertCircleWithStyle(SymbolicCircle, CircleStyle),
  InsertCircleByHistory(Entity, SymbolicCircle, CircleStyle),

  // Remove things
  Remove(Entity),
  RemoveByHistory(Entity),
  RemoveSelected,
  RemoveAll,

  // Update things
  UpdatePoint(Entity, SymbolicPoint, SymbolicPoint), // Entity, before, after
  UpdatePointEnd(Entity, SymbolicPoint, SymbolicPoint), // Entity, before, after
  UpdatePointByHistory(Entity, SymbolicPoint, SymbolicPoint), // Entity, before, after

  // Select/Deselect things
  Select(Entity),
  Deselect(Entity),
  SelectAll,
  DeselectAll,

  // Hide/Unhide things
  Hide(Entity),
  HideByHistory(Entity),
  Unhide(Entity),
  UnhideByHistory(Entity),
  HideSelected,
  UnhideAll,
}

pub type CommandEventChannel = EventChannel<CommandEvent>;

pub type CommandEventReader = ReaderId<CommandEvent>;