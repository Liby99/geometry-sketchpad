use specs::prelude::*;
use crate::{
  resources::{
    styles::DefaultPointStyle,
    geometry::{LastActivePoint, LastActivePointChannel},
    events::{
      GeometryAction, GeometryActionChannel, GeometryActionReader,
      SketchEvent, SketchEventChannel, Geometry, GeometryStyle,
    },
  },
  components::{PointStyle, SymbolicPoint, Selected},
};

pub struct InsertPointSystem {
  geometry_action_reader: Option<GeometryActionReader>,
}

impl Default for InsertPointSystem {
  fn default() -> Self {
    Self { geometry_action_reader: None }
  }
}

impl<'a> System<'a> for InsertPointSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, DefaultPointStyle>,
    Read<'a, GeometryActionChannel>,
    Write<'a, SketchEventChannel>,
    Write<'a, LastActivePointChannel>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_action_reader = Some(world.fetch_mut::<GeometryActionChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    default_point_style,
    geometry_action_channel,
    mut sketch_event_channel,
    mut last_active_point_channel,
    mut sym_points,
    mut point_styles,
    mut selected,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_action_channel.read(reader_id) {
        match event {
          GeometryAction::InsertPoint(sym_point) => {
            let style = default_point_style.get();

            // First create the entity
            let entity = entities.create();
            if let Err(err) = sym_points.insert(entity, *sym_point) { panic!(err) }
            if let Err(err) = point_styles.insert(entity, style) { panic!(err) }
            if let Err(err) = selected.insert(entity, Selected) { panic!(err) }

            // Write the event
            sketch_event_channel.single_write(SketchEvent::Insert(entity, Geometry::Point(*sym_point), GeometryStyle::Point(style)));

            // Mark this created entity as the last active point
            last_active_point_channel.single_write(LastActivePoint::new(entity));
          },
          _ => (),
        }
      }
    }
  }
}