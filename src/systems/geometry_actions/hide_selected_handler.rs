use specs::prelude::*;
use crate::{
  resources::events::{
    GeometryAction, GeometryActionChannel, GeometryActionReader,
    SketchEvent, SketchEventChannel,
  },
  components::{Selected},
};

pub struct HideSelectedHandler {
  geometry_action_reader: Option<GeometryActionReader>,
}

impl Default for HideSelectedHandler {
  fn default() -> Self {
    Self { geometry_action_reader: None }
  }
}

impl<'a> System<'a> for HideSelectedHandler {
  type SystemData = (
    Entities<'a>,
    Read<'a, GeometryActionChannel>,
    Write<'a, SketchEventChannel>,
    ReadStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_action_reader = Some(world.fetch_mut::<GeometryActionChannel>().register_reader());
  }

  fn run(&mut self, (entities, geometry_action_channel, mut sketch_event_channel, selected): Self::SystemData) {
    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_action_channel.read(reader_id) {
        match event {
          GeometryAction::HideSelected => {
            for (entity, _) in (&entities, &selected).join() {
              sketch_event_channel.single_write(SketchEvent::hide(entity));
            }
            break;
          },
          _ => ()
        }
      }
    }
  }
}