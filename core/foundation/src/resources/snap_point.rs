use specs::prelude::*;
use geopad_core_lib::{components::symbolics::*, utilities::*};

pub struct MaybeSnapPoint(Option<SnapPoint>);

impl Default for MaybeSnapPoint {
  fn default() -> Self {
    Self(None)
  }
}

impl MaybeSnapPoint {
  pub fn set(&mut self, snap_point: SnapPoint) {
    self.0 = Some(snap_point);
  }

  pub fn clear(&mut self) {
    self.0 = None;
  }

  pub fn get(&self) -> Option<SnapPoint> {
    self.0
  }
}

#[derive(Debug, Copy, Clone)]
pub struct SnapPoint {
  pub position: ScreenPosition,
  pub symbol: SnapPointType,
}

#[derive(Debug, Copy, Clone)]
pub enum SnapPointType {
  SnapOnPoint(Entity),
  SnapOnLine(Entity, f64), // f64 is t
  SnapOnLineLineIntersection(Entity, Entity), // Line Line
  SnapOnCircle(Entity, f64), // f64 is theta
  // SnapOnCircleLineIntersection(Entity, Entity, CircleIntersectId), // Circle, Line, type
  // SnapOnCircleCircleIntersection(Entity, Entity, CircleIntersectId), // Circle, Circle, type
  NotSnapped,
}