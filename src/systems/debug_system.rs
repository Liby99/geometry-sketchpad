use specs::prelude::*;
use crate::{
  systems::events::{MouseEventChannel, MouseEventReader}
};

pub struct DebugSystem {
  reader_id: Option<MouseEventReader>,
}

impl Default for DebugSystem {
  fn default() -> Self {
    Self { reader_id: None }
  }
}

impl<'a> System<'a> for DebugSystem {
  type SystemData = Read<'a, MouseEventChannel>;

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.reader_id = Some(world.fetch_mut::<MouseEventChannel>().register_reader());
  }

  fn run(&mut self, mouse_event_channel: Self::SystemData) {
    if let Some(reader_id) = &mut self.reader_id {
      for event in mouse_event_channel.read(reader_id) {
        println!("{:?}", event);
      }
    }
  }
}