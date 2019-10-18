use specs::prelude::*;
use crate::{
  resources::events::{
    SketchEventReader, SketchEventChannel, SketchEvent,
  },
  components::Hidden,
};

pub struct HideHandler {
  sketch_event_reader: Option<SketchEventReader>,
}

impl Default for HideHandler {
  fn default() -> Self {
    Self { sketch_event_reader: None }
  }
}

impl<'a> System<'a> for HideHandler {
  type SystemData = (
    Read<'a, SketchEventChannel>,
    WriteStorage<'a, Hidden>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.sketch_event_reader = Some(world.fetch_mut::<SketchEventChannel>().register_reader());
  }

  fn run(&mut self, (sketch_event_channel, mut hidden): Self::SystemData) {
    if let Some(reader_id) = &mut self.sketch_event_reader {
      for event in sketch_event_channel.read(reader_id) {
        match event {
          SketchEvent::Hide(entity, _) => {
            if let Err(err) = hidden.insert(*entity, Hidden) { panic!(err) }
          },
          SketchEvent::Unhide(entity, _) => {
            hidden.remove(*entity);
          },
          _ => (),
        }
      }
    }
  }
}