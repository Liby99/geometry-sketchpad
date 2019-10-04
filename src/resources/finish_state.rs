pub struct FinishState(pub bool);

impl Default for FinishState {
  fn default() -> Self {
    FinishState(false)
  }
}

impl FinishState {
  pub fn not_finished(&self) -> bool {
    !self.0
  }

  pub fn set_finished(&mut self) {
    self.0 = true;
  }
}