pub struct ExitState(bool);

impl Default for ExitState {
  fn default() -> Self {
    Self(false)
  }
}

impl ExitState {
  pub fn is_exiting(&self) -> bool {
    self.0
  }

  pub fn set_need_exit(&mut self) {
    self.0 = true;
  }
}
