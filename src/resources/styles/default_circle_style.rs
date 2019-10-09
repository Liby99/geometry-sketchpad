use crate::utilities::Color;
use crate::components::CircleStyle;

pub struct DefaultCircleStyle(CircleStyle);

impl Default for DefaultCircleStyle {
  fn default() -> Self {
    Self(CircleStyle {
      width: 2.,
      color: Color::new(0.0, 0.6, 0.0, 1.0), // Darker green
    })
  }
}

impl DefaultCircleStyle {
  pub fn get(&self) -> CircleStyle {
    self.0
  }

  #[allow(dead_code)]
  pub fn set(&mut self, circle_style: CircleStyle) {
    self.0 = circle_style;
  }
}