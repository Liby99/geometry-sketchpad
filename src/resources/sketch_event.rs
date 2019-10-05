use specs::prelude::*;

pub enum SketchEvent {
  Inserted(Entity, Geometry),
}

pub enum Geometry {
  Point,
  Line,
}