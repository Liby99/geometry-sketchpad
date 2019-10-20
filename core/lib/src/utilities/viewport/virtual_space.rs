use crate::math::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct VirtualScalar(pub f64);

impl Into<f64> for VirtualScalar {
  fn into(self) -> f64 {
    self.0
  }
}

impl From<f64> for VirtualScalar {
  fn from(f: f64) -> Self {
    Self(f)
  }
}

pub struct VirtualPosition(pub Vector2);

impl Into<Vector2> for VirtualPosition {
  fn into(self) -> Vector2 {
    self.0
  }
}

impl From<Vector2> for VirtualPosition {
  fn from(v: Vector2) -> Self {
    Self(v)
  }
}

impl Add for VirtualPosition {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self(self.0 + other.0)
  }
}

impl Sub for VirtualPosition {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self(self.0 - other.0)
  }
}

impl Neg for VirtualPosition {
  type Output = Self;
  fn neg(self) -> Self {
    Self(-self.0)
  }
}

impl Mul<VirtualScalar> for VirtualPosition {
  type Output = Self;
  fn mul(self, other: VirtualScalar) -> Self {
    Self(self.0 * other.0)
  }
}

impl Mul<VirtualPosition> for VirtualScalar {
  type Output = VirtualPosition;
  fn mul(self, other: VirtualPosition) -> VirtualPosition {
    VirtualPosition(self.0 * other.0)
  }
}

impl Div<VirtualScalar> for VirtualPosition {
  type Output = Self;
  fn div(self, other: VirtualScalar) -> Self {
    Self(self.0 / other.0)
  }
}

pub struct VirtualLine {
  pub from: VirtualPosition,
  pub to: VirtualPosition,
  pub line_type: LineType,
}

impl Into<Line> for VirtualLine {
  fn into(self) -> Line {
    Line {
      from: self.from.into(),
      to: self.to.into(),
      line_type: self.line_type,
    }
  }
}

impl From<Line> for VirtualLine {
  fn from(l: Line) -> Self {
    Self {
      from: l.from.into(),
      to: l.to.into(),
      line_type: l.line_type,
    }
  }
}

pub struct VirtualCircle {
  pub center: VirtualPosition,
  pub radius: VirtualScalar,
}

impl Into<Circle> for VirtualCircle {
  fn into(self) -> Circle {
    Circle {
      center: self.center.into(),
      radius: self.radius.into(),
    }
  }
}

impl From<Circle> for VirtualCircle {
  fn from(c: Circle) -> Self {
    Self {
      center: c.center.into(),
      radius: c.radius.into(),
    }
  }
}