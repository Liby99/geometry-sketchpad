#[derive(Debug, Copy, Clone)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

#[macro_export]
macro_rules! rgb {
  ($r:expr, $g:expr, $b:expr) => {
    Color {
      r: $r,
      g: $g,
      b: $b,
      a: 1.0,
    }
  };
}

#[macro_export]
macro_rules! rgba {
  ($r:expr, $g:expr, $b:expr, $a:expr) => {
    Color {
      r: $r,
      g: $g,
      b: $b,
      a: $a,
    }
  };
}

impl Default for Color {
  fn default() -> Self {
    Self::rgb(0.0, 0.0, 0.0) // Default to black
  }
}

impl Color {
  pub fn rgb(r: f32, g: f32, b: f32) -> Self {
    Self { r, g, b, a: 1.0 }
  }

  pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self { r, g, b, a }
  }

  pub fn apply_alpha(self, a: f32) -> Self {
    Self {
      r: self.r,
      g: self.g,
      b: self.b,
      a: self.a * a,
    }
  }

  pub fn transparent() -> Self {
    rgba!(0.0, 0.0, 0.0, 0.0)
  }

  pub fn white() -> Self {
    rgb!(1.0, 1.0, 1.0)
  }

  pub fn black() -> Self {
    rgb!(0.0, 0.0, 0.0)
  }

  pub fn red() -> Self {
    rgb!(1.0, 0.0, 0.0)
  }

  pub fn green() -> Self {
    rgb!(0.0, 1.0, 0.0)
  }

  pub fn blue() -> Self {
    rgb!(0.0, 0.0, 1.0)
  }

  pub fn magenta() -> Self {
    rgb!(1.0, 0.0, 1.0)
  }
}

impl Into<[f32; 4]> for Color {
  fn into(self) -> [f32; 4] {
    [self.r, self.g, self.b, self.a]
  }
}
