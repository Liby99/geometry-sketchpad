use std::ops::{ Add, Sub, Neg };

pub struct Vec2 {
  x: f64,
  y: f64,
}

impl Add for Vec2 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Sub for Vec2 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl Neg for Vec2 {
  type Output = Vec2;

  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
    }
  }
}