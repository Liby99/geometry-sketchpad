use core_lib::math::Color;

pub fn color_to_hex(color: Color) -> u32 {
  (((color.r * 255.0) as u32) << 16) | (((color.g * 255.0) as u32) << 8) | (color.b * 255.0) as u32
}