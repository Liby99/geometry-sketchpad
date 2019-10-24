use crate::math::*;
use crate::components::styles::*;

#[derive(Debug, Copy, Clone)]
pub struct DefaultCircleStyle(CircleStyle);

impl Default for DefaultCircleStyle {
  fn default() -> Self {
    Self(CircleStyle {
      fill: Color::transparent(),
      border: LineStyle {
        color: rgb!(0.0, 0.6, 0.0),
        width: 2.0,
      },
    })
  }
}

impl DefaultCircleStyle {
  pub fn get(&self) -> CircleStyle {
    self.0
  }
}