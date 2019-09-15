pub struct MouseState {
  pub left_button: ActiveState,
  pub right_button: ActiveState,
  pub in_focus: ActiveState,
  pub abs_pos: [f64; 2],
  pub rel_movement: [f64; 2],
  pub rel_scroll: [f64; 2],
}

impl Default for MouseState {
  fn default() -> Self {
    Self {
      left_button: ActiveState::default(),
      right_button: ActiveState::default(),
      in_focus: ActiveState::default(),
      abs_pos: [0., 0.],
      rel_movement: [0., 0.],
      rel_scroll: [0., 0.],
    }
  }
}

impl MouseState {
  pub fn reset_relative_data(&mut self) {
    self.left_button.reset_relative_data();
    self.right_button.reset_relative_data();
    self.in_focus.reset_relative_data();
    self.rel_movement = [0., 0.];
    self.rel_scroll = [0., 0.];
  }
}

pub struct ActiveState {
  down: bool,
  just_changed: bool,
}

impl Default for ActiveState {
  fn default() -> Self {
    Self { down: false, just_changed: false }
  }
}

impl ActiveState {
  pub fn set(&mut self, next: bool) {
    if self.down != next {
      self.down = next;
      self.just_changed = true;
    }
  }

  pub fn reset_relative_data(&mut self) {
    self.just_changed = false;
  }
}