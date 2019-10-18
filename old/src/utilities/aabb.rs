use super::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

impl AABB {
  pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
    Self { x, y, width, height }
  }

  pub fn contains(&self, p: Vector2) -> bool {
    let Vector2 { x, y } = p;
    self.x <= x && x <= self.x + self.width && self.y <= y && y <= self.y + self.height
  }

  pub fn get_closest_point_to(&self, p: Vector2) -> Vector2 {
    let x = if p.x < self.x {
      self.x
    } else if p.x < self.x + self.width {
      p.x
    } else {
      self.x + self.width
    };
    let y = if p.y < self.y {
      self.y
    } else if p.y < self.y + self.height {
      p.y
    } else {
      self.y + self.height
    };
    vec2![x, y]
  }

  pub fn get_furthest_point_to(&self, p: Vector2) -> Vector2 {
    let x = if p.x < self.x + self.width / 2.0 {
      self.x + self.width
    } else {
      self.x
    };
    let y = if p.y < self.y + self.height / 2.0 {
      self.y + self.height
    } else {
      self.y
    };
    vec2![x, y]
  }
}