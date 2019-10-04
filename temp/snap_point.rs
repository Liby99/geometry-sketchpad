use specs::prelude::*;
use crate::util::Vector2;

pub enum SnapPoint {
  Nothing,
  SnapOnPoint(Entity),
  SnapOnLine(Entity, f32), // f32 is t
  // SnapOnCircle(Entity, f32),
  NotSnapped(Vector2), // Anywhere in the virtual world
}

impl Default for SnapPoint {
  fn default() -> Self {
    Self::Nothing
  }
}