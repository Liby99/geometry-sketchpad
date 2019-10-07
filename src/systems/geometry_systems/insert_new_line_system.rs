use specs::prelude::*;
use crate::{
  resources::{
    styles::DefaultLineStyle,
    events::{
      GeometryAction, GeometryActionChannel, GeometryActionReader,
      SketchEvent, SketchEventChannel, SketchGeometry,
    },
  },
  components::{LineStyle, SymbolicLine, Selected},
};

pub struct InsertNewLineSystem {
  geometry_action_reader: Option<GeometryActionReader>,
}

impl Default for InsertNewLineSystem {
  fn default() -> Self {
    Self { geometry_action_reader: None }
  }
}

impl<'a> System<'a> for InsertNewLineSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, DefaultLineStyle>,
    Read<'a, GeometryActionChannel>,
    Write<'a, SketchEventChannel>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_action_reader = Some(world.fetch_mut::<GeometryActionChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    default_line_style,
    geometry_action_channel,
    mut sketch_event_channel,
    mut sym_lines,
    mut line_styles,
    mut selected,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_action_channel.read(reader_id) {
        match event {
          GeometryAction::InsertLine(sym_line) => {
            let style = default_line_style.get();

            // First create the entity
            let entity = entities.create();
            if let Err(err) = sym_lines.insert(entity, *sym_line) { panic!(err) }
            if let Err(err) = line_styles.insert(entity, style) { panic!(err) }
            if let Err(err) = selected.insert(entity, Selected) { panic!(err) }

            // Write the event
            sketch_event_channel.single_write(SketchEvent::insert(entity, SketchGeometry::Line(*sym_line, style)));
          },
          _ => (),
        }
      }
    }
  }
}