use specs::prelude::*;
use crate::util::Vector2;

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
  pub position: Vector2,
  pub symbo: SnapPointType,
}

#[derive(Debug, Copy, Clone)]
pub enum SnapPointType {
  SnapOnPoint(Entity),
  SnapOnLine(Entity, f64), // f32 is t
  SnapOnIntersection(Entity, Entity),
  SnapOnCircle(Entity, f64), // f32 is theta
  NotSnapped,
}