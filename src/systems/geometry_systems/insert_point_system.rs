use specs::prelude::*;
use crate::{
  resources::{
    styles::DefaultPointStyle,
    geometry::{LastActivePoint, LastActivePointChannel},
    events::{
      InsertEvent, InsertEventChannel, InsertEventReader,
      SketchEvent, SketchEventChannel, Geometry,
    },
  },
  components::{PointStyle, SymbolicPoint, Selected},
};

pub struct InsertPointSystem {
  insert_event_reader: Option<InsertEventReader>,
}

impl Default for InsertPointSystem {
  fn default() -> Self {
    Self { insert_event_reader: None }
  }
}

impl<'a> System<'a> for InsertPointSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, DefaultPointStyle>,
    Read<'a, InsertEventChannel>,
    Write<'a, SketchEventChannel>,
    Write<'a, LastActivePointChannel>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.insert_event_reader = Some(world.fetch_mut::<InsertEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    default_point_style,
    insert_event_channel,
    mut sketch_event_channel,
    mut last_active_point_channel,
    mut sym_points,
    mut point_styles,
    mut selected,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.insert_event_reader {
      for event in insert_event_channel.read(reader_id) {
        match event {
          InsertEvent::Point(sym_point) => {

            // First create the entity
            let entity = entities.create();
            if let Err(err) = sym_points.insert(entity, *sym_point) { panic!(err) }
            if let Err(err) = point_styles.insert(entity, default_point_style.get()) { panic!(err) }
            if let Err(err) = selected.insert(entity, Selected) { panic!(err) }

            // Write the event
            sketch_event_channel.single_write(SketchEvent::Insert(entity, Geometry::Point(*sym_point)));

            // Mark this created entity as the last active point
            last_active_point_channel.single_write(LastActivePoint::new(entity));
          },
          _ => (),
        }
      }
    }
  }
}