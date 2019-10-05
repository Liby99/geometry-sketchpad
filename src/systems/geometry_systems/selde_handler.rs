use specs::prelude::*;
use crate::{
  components::{Selected},
  systems::events::{SketchEventReader, SketchEventChannel, SketchEvent},
};

pub struct SeldeHandler {
  sketch_event_reader: Option<SketchEventReader>,
}

impl Default for SeldeHandler {
  fn default() -> Self {
    Self { sketch_event_reader: None }
  }
}

impl<'a> System<'a> for SeldeHandler {
  type SystemData = (
    Read<'a, SketchEventChannel>,
    WriteStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.sketch_event_reader = Some(world.fetch_mut::<SketchEventChannel>().register_reader());
  }

  fn run(&mut self, (sketch_event_channel, mut selected): Self::SystemData) {
    if let Some(reader_id) = &mut self.sketch_event_reader {
      for event in sketch_event_channel.read(reader_id) {
        match event {
          SketchEvent::Select(entity) => {
            if let Err(err) = selected.insert(*entity, Selected) { panic!(err) };
          },
          SketchEvent::Deselect(entity) => {
            selected.remove(*entity);
          },
          _ => (),
        }
      }
    }
  }
}