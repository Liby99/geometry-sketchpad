use shrev::{EventChannel, ReaderId};
use crate::components::{SymbolicLine, SymbolicPoint};

pub enum InsertEvent {
  Point(SymbolicPoint),
  Line(SymbolicLine),
}

pub type InsertEventChannel = EventChannel<InsertEvent>;

pub type InsertEventReader = ReaderId<InsertEvent>;