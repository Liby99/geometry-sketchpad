use specs::prelude::*;
use crate::{
  utilities::Vector2,
  resources::{Viewport, Tool},
  systems::events::{
    ViewportEvent, ViewportEventChannel,
    ToolChangeEvent, ToolChangeEventChannel, ToolChangeEventReader,
    MouseEvent, MouseEventChannel, MouseEventReader,
  },
};

static DRAG_SPEED : f64 = 2.4;

pub struct MoveViewportViaDrag {
  tool_change_event_reader: Option<ToolChangeEventReader>,
  mouse_event_reader: Option<MouseEventReader>,
}

impl Default for MoveViewportViaDrag {
  fn default() -> Self {
    Self {
      tool_change_event_reader: None,
      mouse_event_reader: None,
    }
  }
}

impl<'a> System<'a> for MoveViewportViaDrag {
  type SystemData = (
    Read<'a, Viewport>,
    Read<'a, ToolChangeEventChannel>,
    Write<'a, MouseEventChannel>,
    Write<'a, ViewportEventChannel>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_event_reader = Some(world.fetch_mut::<ToolChangeEventChannel>().register_reader());
    // self.mouse_event_reader = Some(world.fetch_mut::<MouseEventChannel>().register_reader());
  }

  fn run(&mut self, (
    viewport,
    tool_change_event_channel,
    mut mouse_event_channel,
    mut viewport_event_channel
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.tool_change_event_reader {
      for ToolChangeEvent(tool) in tool_change_event_channel.read(reader_id) {
        match tool {
          Tool::ViewportDrag => {
            self.mouse_event_reader = Some(mouse_event_channel.register_reader());
          },
          _ => {
            if let Some(mouse_reader_id) = &mut self.mouse_event_reader {
              drop(mouse_reader_id);
              self.mouse_event_reader = None;
            }
          }
        }
      }
    }

    if let Some(reader_id) = &mut self.mouse_event_reader {
      for event in mouse_event_channel.read(reader_id) {
        match event {
          MouseEvent::DragMove(delta) => {
            let movement = vec2![-delta.x, delta.y] * DRAG_SPEED * viewport.scale();
            viewport_event_channel.single_write(ViewportEvent::Move(movement));
          },
          _ => (),
        }
      }
    }
  }
}