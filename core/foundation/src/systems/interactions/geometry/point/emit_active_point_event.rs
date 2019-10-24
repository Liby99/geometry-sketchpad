use specs::prelude::*;
use geopad_core_lib::{events::*, utilities::*};
use crate::events::*;

pub struct EmitActivePointEvent {
  geometry_event_reader: Option<GeometryEventReader>,
}

impl Default for EmitActivePointEvent {
  fn default() -> Self {
    Self { geometry_event_reader: None }
  }
}

impl<'a> System<'a> for EmitActivePointEvent {
  type SystemData = (
    Read<'a, GeometryEventChannel>,
    Write<'a, ActivePointEventChannel>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_event_reader = Some(world.fetch_mut::<GeometryEventChannel>().register_reader());
  }

  fn run(&mut self, (
    geometry_event_channel,
    mut active_point_event_channel,
  ): Self::SystemData) {
    if let Some(reader) = &mut self.geometry_event_reader {
      for event in geometry_event_channel.read(reader) {
        match event {
          GeometryEvent::Inserted(ent, Geometry::Point(_, _), _) => {
            active_point_event_channel.single_write(ActivePointEvent(*ent));
          },
          _ => (),
        }
      }
    }
  }
}