use specs::prelude::*;
use crate::components::{SymbolicLine, SymbolicPoint, LineStyle, PointStyle};

pub enum SketchEvent {
  Inserted(Entity, Geometry),
  Removed(Entity, Geometry),
}

pub enum Geometry {
  Point(SymbolicPoint, PointStyle),
  Line(SymbolicLine, LineStyle),
}