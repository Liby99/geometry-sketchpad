use crate::math::*;
use crate::components::styles::*;

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