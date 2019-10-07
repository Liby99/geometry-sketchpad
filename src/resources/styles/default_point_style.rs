use crate::utilities::Color;
use crate::components::PointStyle;

pub struct DefaultPointStyle(PointStyle);

impl Default for DefaultPointStyle {
  fn default() -> Self {
    Self(PointStyle {
      radius: 5.,
      color: Color::red()
    })
  }
}

impl DefaultPointStyle {
  fn get(&self) -> PointStyle {
    self.0
  }

  fn set(&mut self, point_style: PointStyle) {
    self.0 = point_style;
  }
}