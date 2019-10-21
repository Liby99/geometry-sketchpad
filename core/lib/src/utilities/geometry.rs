use crate::components::{symbolics::*, styles::*};

#[derive(Debug, Copy, Clone)]
pub enum Geometry {
  Point(SymbolicPoint, PointStyle),
  Line(SymbolicLine, LineStyle),
  Circle(SymbolicCircle, CircleStyle),
}