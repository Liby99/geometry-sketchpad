use crate::math::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct ScreenScalar(pub f64);

impl Into<f64> for ScreenScalar {
  fn into(self) -> f64 {
    self.0
  }
}

impl From<f64> for ScreenScalar {
  fn from(f: f64) -> Self {
    Self(f)
  }
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenPosition(pub Vector2);

impl Into<Vector2> for ScreenPosition {
  fn into(self) -> Vector2 {
    self.0
  }
}

impl From<Vector2> for ScreenPosition {
  fn from(v: Vector2) -> Self {
    Self(v)
  }
}

impl Add for ScreenPosition {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self(self.0 + other.0)
  }
}

impl Sub for ScreenPosition {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self(self.0 - other.0)
  }
}

impl Neg for ScreenPosition {
  type Output = Self;
  fn neg(self) -> Self {
    Self(-self.0)
  }
}

impl Mul<ScreenScalar> for ScreenPosition {
  type Output = Self;
  fn mul(self, other: ScreenScalar) -> Self {
    Self(self.0 * other.0)
  }
}

impl Mul<ScreenPosition> for ScreenScalar {
  type Output = ScreenPosition;
  fn mul(self, other: ScreenPosition) -> ScreenPosition {
    ScreenPosition(self.0 * other.0)
  }
}

impl Div<ScreenScalar> for ScreenPosition {
  type Output = Self;
  fn div(self, other: ScreenScalar) -> Self {
    Self(self.0 / other.0)
  }
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenLine {
  pub from: ScreenPosition,
  pub to: ScreenPosition,
  pub line_type: LineType,
}

impl Into<Line> for ScreenLine {
  fn into(self) -> Line {
    Line {
      from: self.from.into(),
      to: self.to.into(),
      line_type: self.line_type,
    }
  }
}

impl From<Line> for ScreenLine {
  fn from(l: Line) -> Self {
    Self {
      from: l.from.into(),
      to: l.to.into(),
      line_type: l.line_type,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenCircle {
  pub center: ScreenPosition,
  pub radius: ScreenScalar,
}

impl Into<Circle> for ScreenCircle {
  fn into(self) -> Circle {
    Circle {
      center: self.center.into(),
      radius: self.radius.into(),
    }
  }
}

impl From<Circle> for ScreenCircle {
  fn from(c: Circle) -> Self {
    Self {
      center: c.center.into(),
      radius: c.radius.into(),
    }
  }
}