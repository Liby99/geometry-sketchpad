use specs::prelude::*;
use crate::{
  resources::{ToolState, Tool},
  systems::events::{ViewportEvent, ViewportEventChannel, MouseEvent, MouseEventChannel, MouseEventReader},
};

pub struct MoveViewportViaDrag {
  mouse_event_reader: Option<MouseEventReader>,
}

impl Default for MoveViewportViaDrag {
  fn default() -> Self {
    Self { mouse_event_reader: None }
  }
}

impl<'a> System<'a> for MoveViewportViaDrag {
  type SystemData = (
    Read<'a, ToolState>,
    Read<'a, MouseEventChannel>,
    Write<'a, ViewportEventChannel>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.mouse_event_reader = Some(world.fetch_mut::<MouseEventChannel>().register_reader());
  }

  fn run(&mut self, (tool_state, mouse_event_channel, mut vp_events): Self::SystemData) {
    match tool_state.get() {
      Tool::ViewportDrag => {
        if let Some(reader_id) = &mut self.mouse_event_reader {
          for event in mouse_event_channel.read(reader_id) {
            match event {
              MouseEvent::DragMove(delta) => {
                vp_events.single_write(ViewportEvent::Move(*delta));
              },
              _ => (),
            }
          }
        }
      },
      _ => ()
    }
  }
}