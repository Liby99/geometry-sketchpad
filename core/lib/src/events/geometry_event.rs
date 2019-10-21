use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::utilities::Geometry;

pub enum GeometryEvent {
  Inserted(Entity, Geometry, bool),
  Removed(Entity, Geometry, bool),
  Updated(Entity, Geometry, Geometry, bool),
}

pub type GeometryEventChannel = EventChannel<GeometryEvent>;

pub type GeometryEventReader = ReaderId<GeometryEvent>;