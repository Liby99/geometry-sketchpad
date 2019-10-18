use specs::prelude::*;
use crate::{
  components::*,
  resources::events::{SketchEvent, SketchEventChannel, SketchEventReader, SketchGeometry},
};

pub struct RemoveHandler {
  sketch_event_reader: Option<SketchEventReader>,
}

impl Default for RemoveHandler {
  fn default() -> Self {
    Self { sketch_event_reader: None }
  }
}

impl<'a> System<'a> for RemoveHandler {
  type SystemData = (
    Read<'a, SketchEventChannel>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, Point>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, Line>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, SymbolicCircle>,
    WriteStorage<'a, Circle>,
    WriteStorage<'a, CircleStyle>,
    WriteStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.sketch_event_reader = Some(world.fetch_mut::<SketchEventChannel>().register_reader());
  }

  fn run(&mut self, (
    sketch_event_channel,
    mut sym_points,
    mut points,
    mut point_styles,
    mut sym_lines,
    mut lines,
    mut line_styles,
    mut sym_circles,
    mut circles,
    mut circle_styles,
    mut selected,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.sketch_event_reader {
      for event in sketch_event_channel.read(reader_id) {
        match event {
          SketchEvent::Remove(entity, geom, _) => {
            selected.remove(*entity);
            match geom {
              SketchGeometry::Point(_, _) => {
                sym_points.remove(*entity);
                points.remove(*entity);
                point_styles.remove(*entity);
              },
              SketchGeometry::Line(_, _) => {
                sym_lines.remove(*entity);
                lines.remove(*entity);
                line_styles.remove(*entity);
              },
              SketchGeometry::Circle(_, _) => {
                sym_circles.remove(*entity);
                circles.remove(*entity);
                circle_styles.remove(*entity);
              },
            }
          },
          _ => (),
        }
      }
    }
  }
}