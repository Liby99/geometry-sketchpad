use crate::math::Vector2;

pub static WINDOW_SIZE : [f64; 2] = [960., 720.];

pub struct Viewport {
  pub virtual_center: Vector2,
  pub virtual_size: Vector2,
  pub actual_size: Vector2,
  half_virtual_size: Vector2,
  half_actual_size: Vector2,
}

impl Default for Viewport {
  fn default() -> Self {
    Self::new(vec2![0., 0.], vec2![20., 15.], WINDOW_SIZE.into())
  }
}

impl Viewport {
  pub fn new(virtual_center: Vector2, virtual_size: Vector2, actual_size: Vector2) -> Self {
    Self {
      virtual_center,
      virtual_size: vec2![virtual_size.x, virtual_size.x / actual_size.x * actual_size.y], // Normalize scale
      actual_size,
      half_virtual_size: virtual_size / 2.0,
      half_actual_size: actual_size / 2.0,
    }
  }

  pub fn set(&mut self, window_size: [f64; 2]) {
    self.actual_size = Vector2::from(window_size);
    self.virtual_size.y = self.virtual_size.x / self.actual_size.x * self.actual_size.y;
    self.half_actual_size = self.actual_size / 2.0;
    self.half_virtual_size = self.virtual_size / 2.0;
  }

  pub fn actual_width(&self) -> f64 {
    self.actual_size.x
  }

  pub fn actual_height(&self) -> f64 {
    self.actual_size.y
  }

  pub fn virtual_width(&self) -> f64 {
    self.virtual_size.x
  }

  pub fn virtual_height(&self) -> f64 {
    self.virtual_size.y
  }

  pub fn half_actual_width(&self) -> f64 {
    self.half_actual_size.x
  }

  pub fn half_actual_height(&self) -> f64 {
    self.half_actual_size.y
  }

  pub fn half_virtual_width(&self) -> f64 {
    self.half_virtual_size.x
  }

  pub fn half_virtual_height(&self) -> f64 {
    self.half_virtual_size.y
  }

  pub fn x_min(&self) -> f64 {
    self.virtual_center.x - self.half_virtual_size.x
  }

  pub fn x_max(&self) -> f64 {
    self.virtual_center.x + self.half_virtual_size.x
  }

  pub fn y_min(&self) -> f64 {
    self.virtual_center.y - self.half_virtual_size.y
  }

  pub fn y_max(&self) -> f64 {
    self.virtual_center.y + self.half_virtual_size.y
  }

  pub fn to_actual(&self, point: Vector2) -> [f64; 2] {
    let Vector2 { x, y } = point;
    let x_p = (x - self.virtual_center.x + self.half_virtual_width()) / self.virtual_width() * self.actual_width();
    let y_p = (self.virtual_center.y - y + self.half_virtual_height()) / self.virtual_height() * self.actual_height();
    [x_p, y_p]
  }

  pub fn to_virtual(&self, pos: [f64; 2]) -> Vector2 {
    let [x_p, y_p] = pos;
    let x = (x_p - self.half_actual_width()) / self.actual_width() * self.virtual_width() + self.virtual_center.x;
    let y = (self.half_actual_height() - y_p) / self.actual_height() * self.virtual_height() + self.virtual_center.y;
    vec2![x, y]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_actual() {
    let vp = Viewport::default();
    assert!(vp.to_actual(vec2![0., 0.]) == [480., 360.]);
    assert!(vp.to_actual(vec2![-10., 7.5]) == [0., 0.]);
    assert!(vp.to_actual(vec2![-10., -7.5]) == [0., 720.]);
    assert!(vp.to_actual(vec2![10., 7.5]) == [960., 0.]);
    assert!(vp.to_actual(vec2![10., -7.5]) == [960., 720.]);
    assert!(vp.to_actual(vec2![0., 5.]) == [480., 120.]);
    assert!(vp.to_actual(vec2![5., 5.]) == [720., 120.]);
  }
}