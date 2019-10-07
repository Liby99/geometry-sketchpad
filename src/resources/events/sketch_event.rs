use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::{
  utilities::Vector2,
  components::{SymbolicLine, SymbolicPoint, LineStyle, PointStyle},
};

pub enum SketchEvent {
  Select(Entity),
  Deselect(Entity),
  Insert(Entity, Geometry),
  Remove(Entity, Geometry, GeometryStyle),
  MovePoint(Entity, MovePoint),
}

pub enum Geometry {
  Point(SymbolicPoint),
  Line(SymbolicLine),
}

pub enum GeometryStyle {
  Point(PointStyle),
  Line(LineStyle),
}

pub enum MovePoint {
  Free(Vector2, Vector2), // old_position, new_position
  OnLine(Entity, f64, f64), // line_entity, old_t, new_t
}

pub type SketchEventChannel = EventChannel<SketchEvent>;

pub type SketchEventReader = ReaderId<SketchEvent>;