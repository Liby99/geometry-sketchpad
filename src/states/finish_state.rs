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
}