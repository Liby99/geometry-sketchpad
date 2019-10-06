use specs::prelude::*;

pub struct CreateLineData {
  pub maybe_first_point: Option<Entity>,
}

impl Default for CreateLineData {
  fn default() -> Self {
    Self { maybe_first_point: None }
  }
}