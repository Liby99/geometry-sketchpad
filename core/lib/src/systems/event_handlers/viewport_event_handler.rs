use specs::prelude::*;
use crate::{events::*, resources::*};

pub struct ViewportManager {
  viewport_event_reader: Option<ViewportEventReader>,
}

impl Default for ViewportManager {
  fn default() -> Self {
    Self { viewport_event_reader: None }
  }
}

impl<'a> System<'a> for ViewportManager {
  type SystemData = (
    Read<'a, ViewportEventChannel>,
    Write<'a, Viewport>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.viewport_event_reader = Some(world.fetch_mut::<ViewportEventChannel>().register_reader());
  }

  fn run(&mut self, (viewport_event_channel, mut viewport): Self::SystemData) {
    if let Some(reader) = &mut self.viewport_event_reader {
      for event in viewport_event_channel.read(reader) {
        match event {
          ViewportEvent::Move(new_center) => {
            viewport.virtual_center = *new_center;
          },
          ViewportEvent::Scale(new_size) => {
            viewport.set_screen_size(*new_size);
          },
        }
      }
    }
  }
}