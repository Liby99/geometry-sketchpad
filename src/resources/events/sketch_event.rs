use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::{
  utilities::Vector2,
  components::{SymbolicLine, SymbolicPoint, LineStyle, PointStyle},
};

pub enum SketchEvent {
  Select(Entity),
  Deselect(Entity),
  Insert(Entity, SketchGeometry, bool), // the last bool represents "is_by_history"
  Remove(Entity, SketchGeometry, bool), // the last bool represents "is_by_history"
  MovePoint(Entity, MovePoint),
}

impl SketchEvent {
  pub fn insert(ent: Entity, sketch_geom: SketchGeometry) -> Self {
    Self::Insert(ent, sketch_geom, false)
  }

  pub fn insert_by_history(ent: Entity, sketch_geom: SketchGeometry) -> Self {
    Self::Insert(ent, sketch_geom, true)
  }

  pub fn remove(ent: Entity, sketch_geom: SketchGeometry) -> Self {
    Self::Remove(ent, sketch_geom, false)
  }

  pub fn remove_by_history(ent: Entity, sketch_geom: SketchGeometry) -> Self {
    Self::Remove(ent, sketch_geom, true)
  }
}

#[derive(Debug, Clone, Copy)]
pub enum SketchGeometry {
  Point(SymbolicPoint, PointStyle),
  Line(SymbolicLine, LineStyle),
}

pub enum MovePoint {
  Free(Vector2, Vector2), // old_position, new_position
  OnLine(Entity, f64, f64), // line_entity, old_t, new_t
}

pub type SketchEventChannel = EventChannel<SketchEvent>;

pub type SketchEventReader = ReaderId<SketchEvent>;