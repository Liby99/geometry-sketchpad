use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::utilities::ProcessedGeometry;

pub enum GeometryEvent {
  Inserted(Entity, ProcessedGeometry),
  Removed(Entity, ProcessedGeometry),
  Updated(Entity, ProcessedGeometry, ProcessedGeometry),
}

pub type GeometryEventChannel = EventChannel<GeometryEvent>;

pub type GeometryEventReader = ReaderId<GeometryEvent>;