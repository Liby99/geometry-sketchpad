use crate::components::{symbolics::*, styles::*};

pub enum ProcessedGeometry {
  Point(SymbolicPoint, PointStyle),
  Line(SymbolicLine, LineStyle),
  Circle(SymbolicCircle, CircleStyle),
}

pub enum Geometry {
  Point(SymbolicPoint),
  Line(SymbolicLine),
  Circle(SymbolicCircle),
}