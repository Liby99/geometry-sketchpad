use crate::utilities::Color;
use crate::components::{RectangleStyle, LineStyle};

pub struct DefaultSelectRectangleStyle(RectangleStyle);

impl Default for DefaultSelectRectangleStyle {
  fn default() -> Self {
    Self(RectangleStyle {
      border: LineStyle {
        color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.3 },
        width: 1.,
      },
      fill: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.05 },
    })
  }
}

impl DefaultSelectRectangleStyle {
  pub fn get(&self) -> RectangleStyle {
    self.0
  }

  #[allow(dead_code)]
  pub fn set(&mut self, rect_style: RectangleStyle) {
    self.0 = rect_style;
  }
}