use crate::math::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct VirtualPosition(pub Vector2);

impl VirtualPosition {
    pub fn magnitude(&self) -> VirtualScalar {
        VirtualScalar(self.0.magnitude())
    }
}

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

#[derive(Debug, Clone, Copy)]
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

impl Intersect<VirtualLine> for VirtualLine {
    type Output = Option<VirtualPosition>;

    fn intersect(self, other: Self) -> Self::Output {
        let l1: Line = self.into();
        let l2: Line = other.into();
        l1.intersect(l2).map(VirtualPosition::from)
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum VirtualCircleIntersect {
    TwoPoints(VirtualPosition, VirtualPosition),
    OnePoint(VirtualPosition),
    None,
}

impl From<CircleIntersect> for VirtualCircleIntersect {
    fn from(itsct: CircleIntersect) -> Self {
        match itsct {
            CircleIntersect::TwoPoints(p1, p2) => {
                VirtualCircleIntersect::TwoPoints(p1.into(), p2.into())
            }
            CircleIntersect::OnePoint(p) => VirtualCircleIntersect::OnePoint(p.into()),
            CircleIntersect::None => VirtualCircleIntersect::None,
        }
    }
}

impl Intersect<VirtualCircle> for VirtualCircle {
    type Output = VirtualCircleIntersect;

    fn intersect(self, other: Self) -> Self::Output {
        let c1: Circle = self.into();
        let c2: Circle = other.into();
        c1.intersect(c2).into()
    }
}

impl Intersect<VirtualLine> for VirtualCircle {
    type Output = VirtualCircleIntersect;

    fn intersect(self, other: VirtualLine) -> Self::Output {
        let c: Circle = self.into();
        let l: Line = other.into();
        c.intersect(l).into()
    }
}

impl Intersect<VirtualCircle> for VirtualLine {
    type Output = VirtualCircleIntersect;

    fn intersect(self, other: VirtualCircle) -> Self::Output {
        let l: Line = self.into();
        let c: Circle = other.into();
        c.intersect(l).into()
    }
}
