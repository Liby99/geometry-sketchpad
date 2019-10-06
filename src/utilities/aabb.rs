use super::Vector2;

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

  pub fn contains(&self, p: Vector2) -> bool {
    let Vector2 { x, y } = p;
    self.x <= x && x <= self.x + self.width && self.y <= y && y <= self.y + self.height
  }
}