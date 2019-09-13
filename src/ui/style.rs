use crate::ui::Color;

pub enum LineType {
  Solid,
  Dashed,
  Dotted,
}

pub enum Style {
  PointStyle { radius: f32, color: Color },
  LineStyle { width: f32, color: Color, line_type: LineType },
}