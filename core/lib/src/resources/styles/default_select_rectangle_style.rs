use crate::math::*;
use crate::components::styles::*;

#[derive(Debug, Copy, Clone)]
pub struct DefaultSelectRectangleStyle(RectangleStyle);

impl Default for DefaultSelectRectangleStyle {
  fn default() -> Self {
    Self(RectangleStyle {
      fill: rgba!(0.0, 0.0, 0.0, 0.05),
      border: LineStyle {
        color: rgba!(0.0, 0.0, 0.0, 0.2),
        width: 1.0,
      },
    })
  }
}

impl DefaultSelectRectangleStyle {
  pub fn get(&self) -> RectangleStyle {
    self.0
  }
}