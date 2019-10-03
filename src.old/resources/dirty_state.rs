pub struct DirtyState {
  pub is_sym_elem_dirty: bool,
  pub is_solver_dirty: bool,
  pub is_input_dirty: bool,
  pub is_viewport_dirty: bool,
}

impl Default for DirtyState {
  fn default() -> DirtyState {
    Self {
      is_sym_elem_dirty: true,
      is_solver_dirty: true,
      is_input_dirty: true,
      is_viewport_dirty: true,
    }
  }
}

impl DirtyState {
  pub fn reset(&mut self) {
    self.is_sym_elem_dirty = false;
    self.is_solver_dirty = false;
    self.is_input_dirty = false;
    self.is_viewport_dirty = false;
  }
}