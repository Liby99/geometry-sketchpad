#[derive(Debug, Copy, Clone)]
pub struct Color {
  pub r: f64,
  pub g: f64,
  pub b: f64,
  pub a: f64,
}

#[macro_export]
macro_rules! rgb {
  ($r:expr, $g:expr, $b:expr) => (Color { r: $r, g: $g, b: $b, a: 1.0 });
}

#[macro_export]
macro_rules! rgba {
  ($r:expr, $g:expr, $b:expr, $a:expr) => (Color { r: $r, g: $g, b: $b, a: $a });
}

impl Default for Color {
  fn default() -> Self {
    Self::rgb(0.0, 0.0, 0.0) // Default to black
  }
}

impl Color {
  pub fn rgb(r: f64, g: f64, b: f64) -> Self {
    Self { r, g, b, a: 1.0 }
  }

  pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
    Self { r, g, b, a }
  }

  pub fn transparent() -> Self {
    rgba!(0.0, 0.0, 0.0, 0.0)
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
}