use core_lib::utilities::*;
use shrev::*;

pub enum MouseEvent {
  DragBegin(ScreenPosition),                // Absolute
  DragMove(ScreenPosition, ScreenPosition), // Relative, Absolute
  DragEnd(ScreenPosition),                  // Absolute
  MouseDown(ScreenPosition),
  MouseUp(ScreenPosition),
  Click(ScreenPosition),
}

pub type MouseEventChannel = EventChannel<MouseEvent>;

pub type MouseEventReader = ReaderId<MouseEvent>;
