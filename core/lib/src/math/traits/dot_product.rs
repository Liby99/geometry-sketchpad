use super::super::Vector2;

pub trait DotProduct {
  fn dot(self, other: Self) -> f64;
}

impl DotProduct for Vector2 {
  fn dot(self, other: Self) -> f64 {
    self.x * other.x + self.y * other.y
  }
}
