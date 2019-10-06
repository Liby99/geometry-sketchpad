use std::collections::HashMap;
use crate::util::{Vector2, Key};

pub struct InputState {
  pub mouse_left_button: ActiveState,
  pub mouse_right_button: ActiveState,
  pub mouse_abs_pos: Vector2,
  pub mouse_rel_movement: Vector2,
  pub mouse_pressed_start_point: Option<Vector2>,
  pub rel_scroll: Vector2,
  pub in_focus: ActiveState,
  pub keyboard: Keyboard,
}

impl Default for InputState {
  fn default() -> Self {
    Self {
      mouse_left_button: ActiveState::default(),
      mouse_right_button: ActiveState::default(),
      mouse_abs_pos: vec2![0., 0.],
      mouse_rel_movement: vec2![0., 0.],
      mouse_pressed_start_point: None,
      in_focus: ActiveState::default(),
      rel_scroll: vec2![0., 0.],
      keyboard: Keyboard::default(),
    }
  }
}

impl InputState {
  pub fn reset_relative_data(&mut self) {
    self.mouse_left_button.reset_relative_data();
    self.mouse_right_button.reset_relative_data();
    self.mouse_rel_movement = vec2![0., 0.];
    self.in_focus.reset_relative_data();
    self.rel_scroll = vec2![0., 0.];
    self.keyboard.reset_relative_data();
  }
}

pub struct ActiveState {
  pressed: bool,
  just_changed: bool,
}

impl Default for ActiveState {
  fn default() -> Self {
    Self { pressed: false, just_changed: false }
  }
}

impl ActiveState {
  pub fn new(pressed: bool, just_changed: bool) -> Self {
    Self { pressed, just_changed }
  }

  pub fn set(&mut self, next: bool) {
    if self.pressed != next {
      self.pressed = next;
      self.just_changed = true;
    }
  }

  pub fn is_pressed(&self) -> bool { self.pressed }

  pub fn just_activated(&self) -> bool {
    self.pressed && self.just_changed
  }

  pub fn reset_relative_data(&mut self) {
    self.just_changed = false;
  }
}

pub struct Keyboard {
  keys: HashMap<Key, ActiveState>,
}

impl Default for Keyboard {
  fn default() -> Self {
    Self { keys: HashMap::new() }
  }
}

impl Keyboard {
  pub fn set(&mut self, key: Key, pressed: bool) {
    match self.keys.get_mut(&key) {
      Some(state) => state.set(pressed),
      None => if pressed { self.keys.insert(key, ActiveState::new(true, true)); }
    }
  }

  pub fn is_activated(&self, key: Key) -> bool {
    match self.keys.get(&key) {
      Some(state) => state.pressed,
      None => false,
    }
  }

  pub fn just_activated(&self, key: Key) -> bool {
    match self.keys.get(&key) {
      Some(state) => state.just_activated(),
      None => false,
    }
  }

  pub fn reset_relative_data(&mut self) {
    for (_, state) in self.keys.iter_mut() {
      state.reset_relative_data();
    }
  }
}