use crate::math::{Vector2, Line, AABB};

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

  pub fn virtual_aabb(&self) -> AABB {
    AABB::new(self.x_min(), self.y_min(), self.virtual_width(), self.virtual_height())
  }

  pub fn actual_aabb(&self) -> AABB {
    AABB::new(0., 0., self.actual_width(), self.actual_height())
  }
}

pub trait ViewportTransform {
  type Output;
  fn to_actual(&self, vp: &Viewport) -> Self::Output;
  fn to_virtual(&self, vp: &Viewport) -> Self::Output;
}

impl ViewportTransform for Vector2 {
  type Output = Self;

  fn to_actual(&self, vp: &Viewport) -> Self::Output {
    let Vector2 { x, y } = self;
    let x_p = (x - vp.virtual_center.x + vp.half_virtual_width()) / vp.virtual_width() * vp.actual_width();
    let y_p = (vp.virtual_center.y - y + vp.half_virtual_height()) / vp.virtual_height() * vp.actual_height();
    vec2![x_p, y_p]
  }

  fn to_virtual(&self, vp: &Viewport) -> Self::Output {
    let Vector2 { x: x_p, y: y_p } = self;
    let x = (x_p - vp.half_actual_width()) / vp.actual_width() * vp.virtual_width() + vp.virtual_center.x;
    let y = (vp.half_actual_height() - y_p) / vp.actual_height() * vp.virtual_height() + vp.virtual_center.y;
    vec2![x, y]
  }
}

impl ViewportTransform for Line {
  type Output = Self;

  fn to_actual(&self, vp: &Viewport) -> Self::Output {
    let Line { origin, direction } = self;
    Line { origin: origin.to_actual(vp), direction: -*direction }
  }

  fn to_virtual(&self, vp: &Viewport) -> Self::Output {
    let Line { origin, direction } = self;
    Line { origin: origin.to_virtual(vp), direction: -*direction }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_actual() {
    let vp = &Viewport::default();
    assert!(vec2![0., 0.].to_actual(vp) == vec2![480., 360.]);
    assert!(vec2![0., 0.].to_actual(vp) == vec2![0., 0.]);
    assert!(vec2![0., 0.].to_actual(vp) == vec2![0., 720.]);
    assert!(vec2![0., 0.].to_actual(vp) == vec2![960., 0.]);
    assert!(vec2![0., 0.].to_actual(vp) == vec2![960., 720.]);
    assert!(vec2![0., 0.].to_actual(vp) == vec2![480., 120.]);
    assert!(vec2![0., 0.].to_actual(vp) == vec2![720., 120.]);
  }
}