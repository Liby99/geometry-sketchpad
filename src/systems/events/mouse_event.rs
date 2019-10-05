use shrev::{EventChannel, ReaderId};
use crate::util::Vector2;

#[derive(Debug, Copy, Clone)]
pub enum MouseEvent {
  Click(Vector2),
  // RightClick(Vector2),
  // DoubleClick(Vector2),
  DragBegin(Vector2), // absolute position
  DragMove(Vector2), // relative movement
  DragEnd(Vector2), // absolute position
}

pub type MouseEventChannel = EventChannel<MouseEvent>;

pub type MouseEventReader = ReaderId<MouseEvent>;