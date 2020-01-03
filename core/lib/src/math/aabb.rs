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

  pub fn two_points(p1: Vector2, p2: Vector2) -> Self {
    let diff = p2 - p1;
    Self {
      x: p1.x.min(p2.x),
      y: p1.y.min(p2.y),
      width: diff.x.abs(),
      height: diff.y.abs(),
    }
  }

  pub fn min(&self) -> Vector2 {
    vec2![self.x, self.y]
  }

  pub fn max(&self) -> Vector2 {
    vec2![self.x + self.width, self.y + self.height]
  }

  pub fn x_min(&self) -> f64 {
    self.x
  }

  pub fn x_max(&self) -> f64 {
    self.x + self.width
  }

  pub fn y_min(&self) -> f64 {
    self.y
  }

  pub fn y_max(&self) -> f64 {
    self.y + self.height
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
