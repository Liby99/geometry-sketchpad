use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::{
  utilities::Vector2,
  components::{SymbolicLine, SymbolicPoint, SymbolicCircle, LineStyle, PointStyle, CircleStyle},
};

#[derive(Debug, Copy, Clone)]
pub enum SketchEvent {
  Select(Entity),
  Deselect(Entity),
  Insert(Entity, SketchGeometry, bool), // the last bool represents "is_by_history"
  Remove(Entity, SketchGeometry, bool), // the last bool represents "is_by_history"
  MovePoint(Entity, MovePoint),
  Hide(Entity, bool),
  Unhide(Entity, bool),
}

impl SketchEvent {
  pub fn insert(ent: Entity, sketch_geom: SketchGeometry) -> Self {
    SketchEvent::Insert(ent, sketch_geom, false)
  }

  pub fn insert_by_history(ent: Entity, sketch_geom: SketchGeometry) -> Self {
    SketchEvent::Insert(ent, sketch_geom, true)
  }

  pub fn remove(ent: Entity, sketch_geom: SketchGeometry) -> Self {
    SketchEvent::Remove(ent, sketch_geom, false)
  }

  pub fn remove_by_history(ent: Entity, sketch_geom: SketchGeometry) -> Self {
    SketchEvent::Remove(ent, sketch_geom, true)
  }

  pub fn hide(ent: Entity) -> Self {
    SketchEvent::Hide(ent, false)
  }

  pub fn hide_by_history(ent: Entity) -> Self {
    SketchEvent::Hide(ent, true)
  }

  pub fn unhide(ent: Entity) -> Self {
    SketchEvent::Unhide(ent, false)
  }

  pub fn unhide_by_history(ent: Entity) -> Self {
    SketchEvent::Unhide(ent, true)
  }
}

#[derive(Debug, Clone, Copy)]
pub enum SketchGeometry {
  Point(SymbolicPoint, PointStyle),
  Line(SymbolicLine, LineStyle),
  Circle(SymbolicCircle, CircleStyle),
}

#[derive(Debug, Clone, Copy)]
pub enum MovePoint {
  Free(Vector2, Vector2), // old_position, new_position
  OnLine(Entity, f64, f64), // line_entity, old_t, new_t
  OnCircle(Entity, f64, f64), // circle_entity, old_theta, new_theta
}

pub type SketchEventChannel = EventChannel<SketchEvent>;

pub type SketchEventReader = ReaderId<SketchEvent>;