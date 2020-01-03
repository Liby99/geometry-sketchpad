use crate::components::{styles::*, symbolics::*};

#[derive(Debug, Copy, Clone)]
pub enum Geometry {
    Point(SymbolicPoint, PointStyle),
    Line(SymbolicLine, LineStyle),
    Circle(SymbolicCircle, CircleStyle),
}

#[derive(Debug, Copy, Clone)]
pub enum GeometrySymbol {
    Point(SymbolicPoint),
    Line(SymbolicLine),
    Circle(SymbolicCircle),
}

impl Into<GeometrySymbol> for Geometry {
    fn into(self) -> GeometrySymbol {
        match self {
            Geometry::Point(sym_point, _) => GeometrySymbol::Point(sym_point),
            Geometry::Line(sym_line, _) => GeometrySymbol::Line(sym_line),
            Geometry::Circle(sym_circle, _) => GeometrySymbol::Circle(sym_circle),
        }
    }
}
