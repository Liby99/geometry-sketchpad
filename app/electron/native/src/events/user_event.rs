use core_lib::math::*;
use core_ui::resources::*;

pub enum UserEvent {
  Loop,
  Input(InputEvent),
  Shutdown,
}

pub enum InputEvent {
  Button(ButtonState, Button),
  Motion(MotionEvent)
}

#[derive(PartialEq)]
pub enum ButtonState {
  Press,
  Release,
}

pub enum Button {
  Keyboard(Key),
  Mouse(MouseButton),
}

pub enum MouseButton {
  Left,
  Right,
}

pub enum MotionEvent {
  MouseCursor(Vector2),
  MouseRelative(Vector2),
  MouseScroll(Vector2),
}