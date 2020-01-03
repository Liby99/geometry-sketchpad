use crate::{events::*, resources::*};
use specs::prelude::*;

pub struct ToolStateManager {
  tool_change_event_reader: Option<ToolChangeEventReader>,
}

impl Default for ToolStateManager {
  fn default() -> Self {
    Self {
      tool_change_event_reader: None,
    }
  }
}

impl<'a> System<'a> for ToolStateManager {
  type SystemData = (Read<'a, ToolChangeEventChannel>, Write<'a, ToolState>);

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_event_reader = Some(world.fetch_mut::<ToolChangeEventChannel>().register_reader());
  }

  fn run(&mut self, (tool_change_event_channel, mut tool_state): Self::SystemData) {
    if let Some(reader) = &mut self.tool_change_event_reader {
      for ToolChangeEvent(tool) in tool_change_event_channel.read(reader) {
        tool_state.set(*tool);
      }
    }
  }
}
