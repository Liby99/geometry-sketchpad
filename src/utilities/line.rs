use super::Vector2;

#[derive(Debug, Copy, Clone, Default)]
pub struct Line {
  pub origin: Vector2,
  pub direction: Vector2,
  pub line_type: LineType,
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