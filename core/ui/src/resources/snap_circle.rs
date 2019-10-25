use specs::prelude::*;

pub struct SnapCircle {
  pub maybe_first_point: Option<Entity>,
}

impl Default for SnapCircle {
  fn default() -> Self {
    Self { maybe_first_point: None }
  }
}