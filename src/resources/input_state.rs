pub struct InputState {
  pub mouse_left_button: ActiveState,
  pub mouse_right_button: ActiveState,
  pub mouse_abs_pos: [f64; 2],
  pub mouse_rel_movement: [f64; 2],
  pub rel_scroll: [f64; 2],
  pub in_focus: ActiveState,
}

impl Default for InputState {
  fn default() -> Self {
    Self {
      mouse_left_button: ActiveState::default(),
      mouse_right_button: ActiveState::default(),
      mouse_abs_pos: [0., 0.],
      mouse_rel_movement: [0., 0.],
      in_focus: ActiveState::default(),
      rel_scroll: [0., 0.],
    }
  }
}

impl InputState {
  pub fn reset_relative_data(&mut self) {
    self.mouse_left_button.reset_relative_data();
    self.mouse_right_button.reset_relative_data();
    self.mouse_rel_movement = [0., 0.];
    self.in_focus.reset_relative_data();
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

  pub fn just_activated(&self) -> bool {
    self.down && self.just_changed
  }

  pub fn reset_relative_data(&mut self) {
    self.just_changed = false;
  }
}