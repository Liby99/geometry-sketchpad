use specs::prelude::*;
use crate::{
  resources::events::{SketchEvent, SketchEventChannel, SketchEventReader, SketchGeometry},
  components::{SymbolicLine, LineStyle, SymbolicPoint, PointStyle, SymbolicCircle, CircleStyle},
};

pub struct InsertHistoryGeometry {
  sketch_event_reader: Option<SketchEventReader>,
}

impl Default for InsertHistoryGeometry {
  fn default() -> Self {
    Self { sketch_event_reader: None }
  }
}

impl<'a> System<'a> for InsertHistoryGeometry {
  type SystemData = (
    Read<'a, SketchEventChannel>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, SymbolicCircle>,
    WriteStorage<'a, CircleStyle>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.sketch_event_reader = Some(world.fetch_mut::<SketchEventChannel>().register_reader());
  }

  fn run(&mut self, (
    sketch_event_channel,
    mut sym_points,
    mut point_styles,
    mut sym_lines,
    mut line_styles,
    mut sym_circles,
    mut circle_styles,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.sketch_event_reader {
      for event in sketch_event_channel.read(reader_id) {
        match event {
          SketchEvent::Insert(entity, geometry, true) => {
            match geometry {
              SketchGeometry::Point(sym_point, point_style) => {
                if let Err(err) = sym_points.insert(*entity, *sym_point) { panic!(err) }
                if let Err(err) = point_styles.insert(*entity, *point_style) { panic!(err) }
              },
              SketchGeometry::Line(sym_line, line_style) => {
                if let Err(err) = sym_lines.insert(*entity, *sym_line) { panic!(err) }
                if let Err(err) = line_styles.insert(*entity, *line_style) { panic!(err) }
              },
              SketchGeometry::Circle(sym_circle, circle_style) => {
                if let Err(err) = sym_circles.insert(*entity, *sym_circle) { panic!(err) }
                if let Err(err) = circle_styles.insert(*entity, *circle_style) { panic!(err) }
              },
            }
          },
          _ => (),
        }
      }
    }
  }
}