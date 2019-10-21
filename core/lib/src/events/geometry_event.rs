use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::utilities::Geometry;

pub enum GeometryEvent {
  Inserted(Entity, Geometry),
  Removed(Entity, Geometry),
  Updated(Entity, Geometry, Geometry),
}

pub type GeometryEventChannel = EventChannel<GeometryEvent>;

pub type GeometryEventReader = ReaderId<GeometryEvent>;