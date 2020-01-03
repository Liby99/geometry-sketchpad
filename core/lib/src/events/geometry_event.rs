use crate::{components::symbolics::SymbolicPoint, utilities::Geometry};
use shrev::{EventChannel, ReaderId};
use specs::prelude::*;

pub enum GeometryEvent {
  Inserted(Entity, Geometry, bool),
  Removed(Entity, Geometry, bool),
  PointUpdated(Entity, SymbolicPoint, SymbolicPoint, bool),
  PointUpdateFinished(Entity, SymbolicPoint, SymbolicPoint, bool),
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

  pub fn point_updated(entity: Entity, old_sym_point: SymbolicPoint, new_sym_point: SymbolicPoint) -> Self {
    GeometryEvent::PointUpdated(entity, old_sym_point, new_sym_point, false)
  }

  pub fn point_updated_by_history(entity: Entity, old_sym_point: SymbolicPoint, new_sym_point: SymbolicPoint) -> Self {
    GeometryEvent::PointUpdated(entity, old_sym_point, new_sym_point, true)
  }

  pub fn point_update_finished(entity: Entity, old_sym_point: SymbolicPoint, new_sym_point: SymbolicPoint) -> Self {
    GeometryEvent::PointUpdateFinished(entity, old_sym_point, new_sym_point, false)
  }

  pub fn point_update_finished_by_history(
    entity: Entity,
    old_sym_point: SymbolicPoint,
    new_sym_point: SymbolicPoint,
  ) -> Self {
    GeometryEvent::PointUpdateFinished(entity, old_sym_point, new_sym_point, true)
  }
}
