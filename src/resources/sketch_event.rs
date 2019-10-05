use specs::prelude::*;
use crate::util::Vector2;

pub enum SketchEvent {
  Inserted(Entity, Geometry),
}

pub enum Geometry {
  Point(Vector2),
}