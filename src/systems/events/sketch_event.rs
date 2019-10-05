use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::components::{SymbolicLine, SymbolicPoint, LineStyle, PointStyle};

pub enum SketchEvent {
  Insert(Entity, Geometry),
  Remove(Entity, Geometry),
}

pub enum Geometry {
  Point(SymbolicPoint, PointStyle),
  Line(SymbolicLine, LineStyle),
}

pub type SketchEventChannel = EventChannel<SketchEvent>;

pub type SketchEventReader = ReaderId<SketchEvent>;