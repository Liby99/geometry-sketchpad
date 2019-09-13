pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Color {
  pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    Color { r, g, b, a }
  }
}

impl Into<[f32; 4]> for Color {
  fn into(self) -> [f32; 4] {
    [self.r, self.g, self.b, self.a]
  }
}