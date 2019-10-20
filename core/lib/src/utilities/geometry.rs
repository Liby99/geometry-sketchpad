use crate::components::{symbolics::*, styles::*};

pub enum Geometry {
  Point(SymbolicPoint, PointStyle),
  Line(SymbolicLine, LineStyle),
  Circle(SymbolicCircle, CircleStyle),
}