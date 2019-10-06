use specs::prelude::*;
use crate::{
  components::*,
  resources::{
    events::{
      GeometryAction, GeometryActionReader, GeometryActionChannel,
      SketchEvent, SketchEventChannel
    },
  },
};

pub struct SeldeAllHandler {
  geometry_action_reader: Option<GeometryActionReader>,
}

impl Default for SeldeAllHandler {
  fn default() -> Self {
    Self { geometry_action_reader: None }
  }
}

impl<'a> System<'a> for SeldeAllHandler {
  type SystemData = (
    Entities<'a>,
    Read<'a, GeometryActionChannel>,
    Write<'a, SketchEventChannel>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_action_reader = Some(world.fetch_mut::<GeometryActionChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    geometry_action_channel,
    mut sketch_event_channel,
    sym_points,
    point_styles,
    sym_lines,
    line_styles,
    selected,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_action_channel.read(reader_id) {
        match event {
          GeometryAction::SelectAll => {
            for (entity, _, _, _) in (&entities, &sym_points, &point_styles, !&selected).join() {
              sketch_event_channel.single_write(SketchEvent::Select(entity));
            }
            for (entity, _, _, _) in (&entities, &sym_lines, &line_styles, !&selected).join() {
              sketch_event_channel.single_write(SketchEvent::Select(entity));
            }
          },
          GeometryAction::DeselectAll => {
            for (entity, _, _, _) in (&entities, &sym_points, &point_styles, &selected).join() {
              sketch_event_channel.single_write(SketchEvent::Deselect(entity));
            }
            for (entity, _, _, _) in (&entities, &sym_lines, &line_styles, &selected).join() {
              sketch_event_channel.single_write(SketchEvent::Deselect(entity));
            }
          },
          _ => (),
        }
      }
    }
  }
}