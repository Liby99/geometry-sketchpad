use super::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Line {
  pub origin: Vector2,
  pub direction: Vector2,
}

impl Line {
  pub fn from_to(p1: Vector2, p2: Vector2) -> Self {
    Self {
      origin: p1,
      direction: (p2 - p1).normalized()
    }
  }
}