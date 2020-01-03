use super::LineStyle;
use crate::math::*;
use specs::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct RectangleStyle {
    pub fill: Color,
    pub border: LineStyle,
}

impl Component for RectangleStyle {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
