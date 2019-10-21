use shrev::{EventChannel, ReaderId};
use crate::math::*;

pub enum ViewportEvent {
  Move(Vector2), // Virtual Center
  Scale(Vector2), // Virtual Size
}

pub type ViewportEventChannel = EventChannel<ViewportEvent>;

pub type ViewportEventReader = ReaderId<ViewportEvent>;