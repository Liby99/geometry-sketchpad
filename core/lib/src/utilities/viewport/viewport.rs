use crate::math::*;

pub static WINDOW_SIZE : [f64; 2] = [960., 720.];

#[derive(Debug, Clone, Copy)]
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

  pub fn set_window_size(&mut self, window_size: Vector2) {
    self.actual_size = window_size;
    self.virtual_size.y = self.virtual_size.x / self.actual_size.x * self.actual_size.y;
    self.half_actual_size = self.actual_size / 2.0;
    self.half_virtual_size = self.virtual_size / 2.0;
  }

  pub fn virtual_to_actual_scale(&self) -> f64 {
    self.virtual_size.x / self.actual_size.x
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