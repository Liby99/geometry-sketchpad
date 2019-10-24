use shrev::*;
use geopad_core_lib::utilities::*;

pub enum MouseEvent {
  DragBegin(ScreenPosition), // Absolute
  DragMove(ScreenPosition, ScreenPosition), // Relative, Absolute
  DragEnd(ScreenPosition), // Absolute
  MouseDown(ScreenPosition),
  MouseUp(ScreenPosition),
  Click(ScreenPosition),
}

pub type MouseEventChannel = EventChannel<MouseEvent>;

pub type MouseEventReader = ReaderId<MouseEvent>;