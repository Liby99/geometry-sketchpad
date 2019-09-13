use std::ops::{ Add, Sub, Neg, Mul, Div };

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
  pub x: f64,
  pub y: f64,
}

impl Vec2 {
  pub fn magnitude(self) -> f64 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  pub fn normalized(self) -> Self {
    self / self.magnitude()
  }
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

impl Mul<f64> for Vec2 {
  type Output = Self;

  fn mul(self, other: f64) -> Self {
    Self {
      x: self.x * other,
      y: self.y * other,
    }
  }
}

impl Div<f64> for Vec2 {
  type Output = Self;

  fn div(self, other: f64) -> Self {
    Self {
      x: self.x / other,
      y: self.y / other,
    }
  }
}

impl Mul<Vec2> for f64 {
  type Output = Vec2;

  fn mul(self, Vec2 { x, y }: Vec2) -> Self::Output {
    Vec2 {
      x: self * x,
      y: self * y,
    }
  }
}