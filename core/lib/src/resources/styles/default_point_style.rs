use crate::components::styles::*;
use crate::math::*;

#[derive(Debug, Copy, Clone)]
pub struct DefaultPointStyle(PointStyle);

impl Default for DefaultPointStyle {
    fn default() -> Self {
        Self(PointStyle {
            color: Color::red(),
            radius: 5.0,
            border_color: Color::black(),
            border_width: 1.5,
        })
    }
}

impl DefaultPointStyle {
    pub fn get(&self) -> PointStyle {
        self.0
    }
}
