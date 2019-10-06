#[derive(Debug, Copy, Clone)]
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

  pub fn red() -> Self {
    Self::new(1.0, 0.0, 0.0, 1.0)
  }

  #[allow(dead_code)]
  pub fn green() -> Self {
    Self::new(0.0, 1.0, 0.0, 1.0)
  }

  pub fn blue() -> Self {
    Self::new(0.0, 0.0, 1.0, 1.0)
  }

  #[allow(dead_code)]
  pub fn black() -> Self {
    Self::new(0.0, 0.0, 0.0, 1.0)
  }

  pub fn white() -> Self {
    Self::new(1.0, 1.0, 1.0, 1.0)
  }

  pub fn magenta() -> Self {
    Self::new(1.0, 0.0, 1.0, 1.0)
  }
}

impl Into<[f32; 4]> for Color {
  fn into(self) -> [f32; 4] {
    [self.r, self.g, self.b, self.a]
  }
}