use core_lib::math::*;

pub enum UserEvent {
  Loop(f64), // dt
  Input(InputEvent),
  Shutdown,
}

pub enum InputEvent {
  Button(ButtonState, Button),
  Motion(MotionEvent)
}

pub enum ButtonState {
  Press,
  Release,
}

pub enum Button {
  Keyboard(Key),
  Mouse(MouseButton),
}

pub enum Key {
  A,
  B,
  C,
  D,
}

pub enum MouseButton {
  Left,
  Right,
  Middle,
}

pub enum MotionEvent {
  MouseCursor(Vector2),
  MouseRelative(Vector2),
  MouseScroll(Vector2),
}