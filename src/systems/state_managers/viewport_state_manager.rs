use specs::prelude::*;
use crate::{
  resources::{Viewport},
  systems::events::{ViewportEvent, ViewportEventChannel, ViewportEventReader},
};

static SPEED_ADJUSTMENT : f64 = 20.0;

pub struct ViewportStateManager {
  viewport_event_reader: Option<ViewportEventReader>,
}

impl Default for ViewportStateManager {
  fn default() -> Self {
    Self { viewport_event_reader: None }
  }
}

impl<'a> System<'a> for ViewportStateManager {
  type SystemData = (
    Read<'a, ViewportEventChannel>,
    Write<'a, Viewport>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.viewport_event_reader = Some(world.fetch_mut::<ViewportEventChannel>().register_reader());
  }

  fn run(&mut self, (
    viewport_event_channel,
    mut viewport,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.viewport_event_reader {
      for event in viewport_event_channel.read(reader_id) {
        match event {
          ViewportEvent::Move(delta) => {
            let virtual_to_actual_scale = viewport.scale();
            viewport.virtual_center += *delta * virtual_to_actual_scale * SPEED_ADJUSTMENT;
          },
          ViewportEvent::Resize(window_size) => {
            viewport.set_window_size(*window_size);
          },
        }
      }
    } else {
      panic!("[viewport_state_manager] No reader id");
    }
  }
}