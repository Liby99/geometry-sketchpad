use specs::prelude::*;
use crate::{
  resources::{
    styles::DefaultCircleStyle,
    events::{
      GeometryAction, GeometryActionChannel, GeometryActionReader,
      SketchEvent, SketchEventChannel, SketchGeometry,
    },
  },
  components::{CircleStyle, SymbolicCircle, Selected},
};

pub struct InsertNewCircleSystem {
  geometry_action_reader: Option<GeometryActionReader>,
}

impl Default for InsertNewCircleSystem {
  fn default() -> Self {
    Self { geometry_action_reader: None }
  }
}

impl<'a> System<'a> for InsertNewCircleSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, DefaultCircleStyle>,
    Read<'a, GeometryActionChannel>,
    Write<'a, SketchEventChannel>,
    WriteStorage<'a, SymbolicCircle>,
    WriteStorage<'a, CircleStyle>,
    WriteStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_action_reader = Some(world.fetch_mut::<GeometryActionChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    default_circle_style,
    geometry_action_channel,
    mut sketch_event_channel,
    mut sym_circles,
    mut circle_styles,
    mut selected,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_action_channel.read(reader_id) {
        match event {
          GeometryAction::InsertCircle(sym_circle) => {
            let style = default_circle_style.get();

            // First create the entity
            let entity = entities.create();
            if let Err(err) = sym_circles.insert(entity, *sym_circle) { panic!(err) }
            if let Err(err) = circle_styles.insert(entity, style) { panic!(err) }
            if let Err(err) = selected.insert(entity, Selected) { panic!(err) }

            // Write the event
            sketch_event_channel.single_write(SketchEvent::insert(entity, SketchGeometry::Circle(*sym_circle, style)));
          },
          _ => (),
        }
      }
    }
  }
}