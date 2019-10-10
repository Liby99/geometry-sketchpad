use specs::prelude::*;
use crate::{
  resources::{
    events::{
      GeometryAction, GeometryActionChannel, GeometryActionReader,
      SketchEvent, SketchEventChannel
    },
  },
  components::Hidden,
};

pub struct UnhideAllHandler {
  geometry_action_reader: Option<GeometryActionReader>,
}

impl Default for UnhideAllHandler {
  fn default() -> Self {
    Self { geometry_action_reader: None }
  }
}

impl<'a> System<'a> for UnhideAllHandler {
  type SystemData = (
    Entities<'a>,
    Read<'a, GeometryActionChannel>,
    Write<'a, SketchEventChannel>,
    ReadStorage<'a, Hidden>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_action_reader = Some(world.fetch_mut::<GeometryActionChannel>().register_reader());
  }

  fn run(&mut self, (entities, geometry_actiona_channel, mut sketch_event_channel, hidden): Self::SystemData) {
    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_actiona_channel.read(reader_id) {
        match event {
          GeometryAction::UnhideAll => {
            for (entity, _) in (&entities, &hidden).join() {
              sketch_event_channel.single_write(SketchEvent::unhide(entity));
            }
            break;
          },
          _ => (),
        }
      }
    }
  }
}