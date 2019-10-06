use shrev::{EventChannel, ReaderId};
use crate::utilities::Vector2;

#[derive(Debug, Copy, Clone)]
pub enum MouseEvent {
  MouseDown(Vector2),
  Click(Vector2),
  // RightClick(Vector2),
  // DoubleClick(Vector2),
  DragBegin(Vector2), // absolute position
  DragMove(Vector2), // relative movement
  DragEnd(Vector2), // absolute position
}

pub type MouseEventChannel = EventChannel<MouseEvent>;

pub type MouseEventReader = ReaderId<MouseEvent>;