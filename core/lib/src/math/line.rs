use super::{Vector2, DotProduct, Project};

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

  pub fn rel_t_of_point(&self, p: Vector2) -> f64 {
    self.t_of_point(p) / self.from_to_length()
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
    let proj = p.project(*self);
    let t = self.t_of_point(proj);
    match self.line_type {
      LineType::Straight => proj,
      LineType::Ray => if t >= 0.0 { proj } else { self.from },
      LineType::Segment => if t >= 0.0 { if t <= self.from_to_length() { proj } else { self.to } } else { self.from }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_line_get_closest_point() {
    let l = Line { from: vec2![0., 0.], to: vec2![1., 0.], line_type: LineType::Straight };
    for i in -10..10 {
      let p = vec2![i as f64, i as f64];
      let proj = l.get_closest_point(p);
      assert!(proj.x == p.x, "Expected: {}, Actual: {}", p.x, proj.x);
    }
  }
}