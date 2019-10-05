use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::{
  systems::events::ToolChangeEvent,
  resources::ToolState,
};

pub struct ToolStateManager {
  tool_change_event_reader_id: Option<ReaderId<ToolChangeEvent>>,
}

impl Default for ToolStateManager {
  fn default() -> Self {
    Self { tool_change_event_reader_id: None }
  }
}

impl<'a> System<'a> for ToolStateManager {
  type SystemData = (
    Read<'a, EventChannel<ToolChangeEvent>>,
    Write<'a, ToolState>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_event_reader_id = Some(world.fetch_mut::<EventChannel<ToolChangeEvent>>().register_reader());
  }

  fn run(&mut self, (tool_change_event_channel, mut tool_state): Self::SystemData) {
    if let Some(reader_id) = &mut self.tool_change_event_reader_id {
      for ToolChangeEvent(tool) in tool_change_event_channel.read(reader_id) {
        tool_state.set(*tool);
      }
    } else {
      panic!("[tool_state_manager] No tool change event reader id");
    }
  }
}