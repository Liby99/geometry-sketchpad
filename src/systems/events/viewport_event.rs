use shrev::{EventChannel, ReaderId};
use crate::util::Vector2;

pub enum ViewportEvent {
  Move(Vector2),
  Resize(Vector2),
}

pub type ViewportEventChannel = EventChannel<ViewportEvent>;

pub type ViewportEventReader = ReaderId<ViewportEvent>;