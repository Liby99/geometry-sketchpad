pub struct DirtyState {
  pub is_solver_dirty: bool,
  pub is_input_dirty: bool,
}

impl Default for DirtyState {
  fn default() -> DirtyState {
    Self {
      is_solver_dirty: true,
      is_input_dirty: true,
    }
  }
}

impl DirtyState {
  pub fn reset(&mut self) {
    self.is_solver_dirty = false;
    self.is_input_dirty = false;
  }
}