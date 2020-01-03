use crate::{events::*, resources::*};
use core_lib::{events::*, math::*};
use specs::prelude::*;

static SPEED: f64 = 1.0;

pub struct MoveViewportViaScroll {
  tool_change_event_reader: Option<ToolChangeEventReader>,
  can_scroll: bool,
}

impl Default for MoveViewportViaScroll {
  fn default() -> Self {
    Self {
      tool_change_event_reader: None,
      can_scroll: false,
    }
  }
}

impl<'a> System<'a> for MoveViewportViaScroll {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, DeltaTime>,
    Read<'a, ToolChangeEventChannel>,
    Write<'a, ViewportEventChannel>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_event_reader = Some(world.fetch_mut::<ToolChangeEventChannel>().register_reader());
  }

  fn run(
    &mut self,
    (input_state, delta_time, tool_change_event_channel, mut viewport_event_channel): Self::SystemData,
  ) {
    if let Some(reader) = &mut self.tool_change_event_reader {
      for ToolChangeEvent(tool) in tool_change_event_channel.read(reader) {
        self.can_scroll = match tool {
          Tool::Viewport => false,
          _ => true,
        };
      }
    }

    if self.can_scroll {
      if !input_state.rel_scroll.is_zero() {
        let raw_movement = input_state.rel_scroll * delta_time.get() * SPEED;
        let movement = if cfg!(target_os = "macos") {
          vec2![-raw_movement.x, raw_movement.y]
        } else {
          raw_movement
        };
        viewport_event_channel.single_write(ViewportEvent::Move(movement));
      }
    }
  }
}
