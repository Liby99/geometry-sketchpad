use super::{Vector2, Position, Direction};
use super::traits::DotProduct;

#[derive(Debug, Copy, Clone, Default)]
pub struct Line {
  pub origin: Position,
  pub direction: Direction,
  pub line_type: LineType,
}

#[macro_export]
macro_rules! line {
  ($o : expr, $d : expr, $t : expr) => (Line {
    origin: Position($o),
    direction: Direction($d),
    line_type: $t,
  })
}

impl Line {
  pub fn point_is_on_line(&self, p: Vector2) -> bool {
    let t = (p - self.origin).dot(self.direction);
    match self.line_type {
      LineType::Line => true,
      LineType::Ray => t >= 0.0,
      LineType::Segment(max_t) => t >= 0.0 && t <= max_t,
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub enum LineType {
  Line,
  Ray,
  Segment(f64), // Max t
}

impl Default for LineType {
  fn default() -> Self {
    LineType::Line
  }
}