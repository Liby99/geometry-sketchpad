use specs::prelude::*;
use crate::resources::{InputState, Tool, events::{ToolChangeEventChannel, ToolChangeEvent}};
use shrev::{EventChannel, ReaderId};

enum GuiSystemAction {
  ToolChange(Tool)
}

type GuiSystemActionChannel = EventChannel<GuiSystemAction>;
type GuiSystemActionReader = ReaderId<GuiSystemAction>;

pub struct GuiSystem {
  init: bool,
  gui_action_reader: Option<GuiSystemActionReader>,
}

impl Default for GuiSystem {
  fn default() -> Self {
    Self {
      init: false,
      gui_action_reader: None,
    }
  }
}

impl<'a> System<'a> for GuiSystem {
  type SystemData = ();

  fn run(&mut self, data: Self::SystemData) {
  }
}
