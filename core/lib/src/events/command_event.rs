use specs::prelude::*;
use crate::components::symbolics::*;

pub enum CommandEvent {

  // Insert things
  InsertPoint(SymbolicPoint),
  InsertLine(SymbolicLine),
  InsertCircle(SymbolicCircle),

  // Remove things
  Remove(Entity),
  RemoveSelected,

  // Update things
  UpdatePoint(Entity, SymbolicPoint, SymbolicPoint), // Entity, before, after
  DragPointEnd(Entity, SymbolicPoint, SymbolicPoint), // Entity, before, after

  // Select/Deselect things
  Select(Entity),
  Deselect(Entity),
  SelectAll,
  DeselectAll,

  // Hide/Unhide things
  Hide(Entity),
  Unhide(Entity),
  HideSelected,
  UnhideAll,
}