use crate::components::styles::*;
use crate::math::*;

#[derive(Debug, Copy, Clone)]
pub struct DefaultLineStyle(LineStyle);

impl Default for DefaultLineStyle {
  fn default() -> Self {
    Self(LineStyle {
      color: Color::blue(),
      width: 2.0,
    })
  }
}

impl DefaultLineStyle {
  pub fn get(&self) -> LineStyle {
    self.0
  }
}
