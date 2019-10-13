use super::Vector2;

#[derive(Debug, Copy, Clone, Default)]
pub struct Line {
  pub origin: Vector2,
  pub direction: Vector2,
  pub line_type: LineType,
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