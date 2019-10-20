use crate::math::*;
use crate::components::styles::*;

#[derive(Debug, Copy, Clone)]
pub struct DefaultCircleStyle(CircleStyle);

impl Default for DefaultCircleStyle {
  fn default() -> Self {
    Self(CircleStyle {
      fill: Color::transparent(),
      border: LineStyle {
        color: Color::green(),
        width: 2.0,
      },
    })
  }
}