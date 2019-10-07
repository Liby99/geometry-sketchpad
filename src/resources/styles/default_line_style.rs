use crate::utilities::Color;
use crate::components::LineStyle;

pub struct DefaultLineStyle(LineStyle);

impl Default for DefaultLineStyle {
  fn default() -> Self {
    Self(LineStyle {
      width: 2.,
      color: Color::blue()
    })
  }
}

impl DefaultLineStyle {
  pub fn get(&self) -> LineStyle {
    self.0
  }

  pub fn set(&mut self, line_style: LineStyle) {
    self.0 = line_style;
  }
}