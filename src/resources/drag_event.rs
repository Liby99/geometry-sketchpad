use crate::util::Vector2;

pub enum DragEvent {
  Begin,
  Move(Vector2),
  End,
}