use specs::prelude::*;

pub struct SnapLine {
  pub maybe_first_point: Option<Entity>,
}

impl Default for SnapLine {
  fn default() -> Self {
    Self { maybe_first_point: None }
  }
}