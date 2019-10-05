use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::{
  resources::{SketchEvent},
  components::{SymbolicPoint, Point, PointStyle, SymbolicLine, Line, LineStyle, Selected},
};

pub struct RemoveHandler {
  sketch_event_reader: Option<ReaderId<SketchEvent>>,
}

impl Default for RemoveHandler {
  fn default() -> Self {
    Self { sketch_event_reader: None }
  }
}

impl<'a> System<'a> for RemoveHandler {
  type SystemData = (
    Read<'a, EventChannel<SketchEvent>>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, Point>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, Line>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.sketch_event_reader = Some(world.fetch_mut::<EventChannel<SketchEvent>>().register_reader());
  }

  fn run(&mut self, (
    sketch_event_channel,
    mut sym_points,
    mut points,
    mut point_styles,
    mut sym_lines,
    mut lines,
    mut line_styles,
    mut selected,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.sketch_event_reader {
      for event in sketch_event_channel.read(reader_id) {
        match event {
          SketchEvent::Removed(entity, _) => {
            sym_points.remove(*entity);
            points.remove(*entity);
            point_styles.remove(*entity);
            sym_lines.remove(*entity);
            lines.remove(*entity);
            line_styles.remove(*entity);
            selected.remove(*entity);
          },
          _ => (),
        }
      }
    }
  }
}