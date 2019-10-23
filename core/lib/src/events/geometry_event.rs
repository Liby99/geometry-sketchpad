use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::utilities::Geometry;

pub enum GeometryEvent {
  Inserted(Entity, Geometry, bool),
  Removed(Entity, Geometry, bool),
  Updated(Entity, Geometry, Geometry, bool),
  UpdateFinished(Entity, Geometry, Geometry, bool),
}

pub type GeometryEventChannel = EventChannel<GeometryEvent>;

pub type GeometryEventReader = ReaderId<GeometryEvent>;

impl GeometryEvent {
  pub fn inserted(entity: Entity, geometry: Geometry) -> Self {
    GeometryEvent::Inserted(entity, geometry, false)
  }

  pub fn inserted_by_history(entity: Entity, geometry: Geometry) -> Self {
    GeometryEvent::Inserted(entity, geometry, true)
  }

  pub fn removed(entity: Entity, geometry: Geometry) -> Self {
    GeometryEvent::Removed(entity, geometry, false)
  }

  pub fn removed_by_history(entity: Entity, geometry: Geometry) -> Self {
    GeometryEvent::Removed(entity, geometry, true)
  }

  pub fn updated(entity: Entity, old_geom: Geometry, new_geom: Geometry) -> Self {
    GeometryEvent::Updated(entity, old_geom, new_geom, false)
  }

  pub fn updated_by_history(entity: Entity, old_geom: Geometry, new_geom: Geometry) -> Self {
    GeometryEvent::Updated(entity, old_geom, new_geom, true)
  }

  pub fn update_finished(entity: Entity, old_geom: Geometry, new_geom: Geometry) -> Self {
    GeometryEvent::UpdateFinished(entity, old_geom, new_geom, false)
  }

  pub fn update_finished_by_history(entity: Entity, old_geom: Geometry, new_geom: Geometry) -> Self {
    GeometryEvent::UpdateFinished(entity, old_geom, new_geom, true)
  }
}