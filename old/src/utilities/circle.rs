use super::Vector2;

#[derive(Copy, Clone, Debug)]
pub struct Circle {
  pub center: Vector2,
  pub radius: f64,
}

impl Circle {
  pub fn new(center: Vector2, radius: f64) -> Self {
    Self { center, radius }
  }
}