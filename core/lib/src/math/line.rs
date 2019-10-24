use super::{Vector2, DotProduct};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line {
  pub from: Vector2,
  pub to: Vector2,
  pub line_type: LineType,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LineType {
  Straight,
  Ray,
  Segment,
}

impl Line {
  pub fn direction(&self) -> Vector2 {
    (self.to - self.from).normalized()
  }

  pub fn from_to_length(&self) -> f64 {
    (self.to - self.from).magnitude()
  }

  pub fn t_of_point(&self, p: Vector2) -> f64 {
    (p - self.from).dot(self.direction())
  }

  pub fn point_at_t(&self, t: f64) -> Vector2 {
    self.from + self.direction() * t
  }

  pub fn point_is_on_line(&self, p: Vector2) -> bool {
    let t = self.t_of_point(p);
    match self.line_type {
      LineType::Straight => true,
      LineType::Ray => t >= 0.0,
      LineType::Segment => 0.0 <= t && t <= self.from_to_length(),
    }
  }

  pub fn get_closest_point(&self, p: Vector2) -> Vector2 {
    let t = self.t_of_point(p);
    let t = match self.line_type {
      LineType::Straight => t,
      LineType::Ray => t.max(0.0),
      LineType::Segment => t.max(0.0).min(self.from_to_length()),
    };
    self.from + t * self.direction()
  }
}