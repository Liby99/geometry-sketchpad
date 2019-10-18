pub struct DeltaTime(f64);

impl Default for DeltaTime {
  fn default() -> Self {
    Self(0.016)
  }
}

impl DeltaTime {
  pub fn set(&mut self, dt: f64) {
    self.0 = dt;
  }

  pub fn get(&self) -> f64 {
    self.0
  }
}