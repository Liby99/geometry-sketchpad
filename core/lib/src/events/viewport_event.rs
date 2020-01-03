use crate::math::*;
use shrev::{EventChannel, ReaderId};

pub enum ViewportEvent {
    Move(Vector2),   // Virtual Center
    Scale(f64),      // Change in pixel
    Resize(Vector2), // Screen Size
}

pub type ViewportEventChannel = EventChannel<ViewportEvent>;

pub type ViewportEventReader = ReaderId<ViewportEvent>;
