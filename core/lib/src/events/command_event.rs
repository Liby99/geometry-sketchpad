use crate::components::{styles::*, symbolics::*};
use shrev::*;
use specs::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct CommandEvent {
  pub command: Command,
  pub event_id: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
  PointInsert(InsertPointEvent),
  LineInsert(InsertLineEvent),
  CircleInsert(InsertCircleEvent),
  Remove(RemoveEvent),
  Update(UpdateEvent),
  Select(SelectEvent),
  Hide(HideEvent),
}

#[derive(Debug, Clone, Copy)]
pub enum InsertPointEvent {
  InsertPoint(SymbolicPoint),
  InsertMidPointFromSelection,
  InsertPointWithStyle(SymbolicPoint, PointStyle),
  InsertPointByHistory(Entity, SymbolicPoint, PointStyle),
}

#[derive(Debug, Clone, Copy)]
pub enum InsertLineEvent {
  InsertLine(SymbolicLine),
  InsertParallelFromSelection,
  InsertPerpendicularFromSelection,
  InsertLineWithStyle(SymbolicLine, LineStyle),
  InsertLineByHistory(Entity, SymbolicLine, LineStyle),
}

#[derive(Debug, Clone, Copy)]
pub enum InsertCircleEvent {
  InsertCircle(SymbolicCircle),
  InsertCircleWithStyle(SymbolicCircle, CircleStyle),
  InsertCircleByHistory(Entity, SymbolicCircle, CircleStyle),
}

#[derive(Debug, Clone, Copy)]
pub enum RemoveEvent {
  Remove(Entity),
  RemoveByHistory(Entity),
  RemoveSelected,
  RemoveAll,
}

#[derive(Debug, Clone, Copy)]
pub enum UpdateEvent {
  UpdatePoint(Entity, SymbolicPoint, SymbolicPoint), // Entity, before, after
  UpdatePointEnd(Entity, SymbolicPoint, SymbolicPoint), // Entity, before, after
  UpdatePointByHistory(Entity, SymbolicPoint, SymbolicPoint), // Entity, before, after
}

#[derive(Debug, Clone, Copy)]
pub enum SelectEvent {
  Select(Entity),
  Deselect(Entity),
  SelectAll,
  DeselectAll,
}

#[derive(Debug, Clone, Copy)]
pub enum HideEvent {
  Hide(Entity),
  HideByHistory(Entity),
  Unhide(Entity),
  UnhideByHistory(Entity),
  HideSelected,
  UnhideAll,
}

pub type CommandEventChannel = EventChannel<CommandEvent>;

pub type CommandEventReader = ReaderId<CommandEvent>;
