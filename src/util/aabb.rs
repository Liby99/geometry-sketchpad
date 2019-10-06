#[derive(Debug, Copy, Clone)]
pub struct AABB {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

impl AABB {
  pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
    Self { x, y, width, height }
  }
}