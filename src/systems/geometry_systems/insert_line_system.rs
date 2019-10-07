use specs::prelude::*;
use crate::{
  resources::{
    styles::DefaultLineStyle,
    events::{
      InsertEvent, InsertEventChannel, InsertEventReader,
      SketchEvent, SketchEventChannel, Geometry,
    },
  },
  components::{LineStyle, SymbolicLine, Selected},
};

pub struct InsertLineSystem {
  insert_event_reader: Option<InsertEventReader>,
}

impl Default for InsertLineSystem {
  fn default() -> Self {
    Self { insert_event_reader: None }
  }
}

impl<'a> System<'a> for InsertLineSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, DefaultLineStyle>,
    Read<'a, InsertEventChannel>,
    Write<'a, SketchEventChannel>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.insert_event_reader = Some(world.fetch_mut::<InsertEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    default_line_style,
    insert_event_channel,
    mut sketch_event_channel,
    mut sym_lines,
    mut line_styles,
    mut selected,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.insert_event_reader {
      for event in insert_event_channel.read(reader_id) {
        match event {
          InsertEvent::Line(sym_line) => {

            // First create the entity
            let entity = entities.create();
            if let Err(err) = sym_lines.insert(entity, *sym_line) { panic!(err) }
            if let Err(err) = line_styles.insert(entity, default_line_style.get()) { panic!(err) }
            if let Err(err) = selected.insert(entity, Selected) { panic!(err) }

            // Write the event
            sketch_event_channel.single_write(SketchEvent::Insert(entity, Geometry::Line(*sym_line)));
          },
          _ => (),
        }
      }
    }
  }
}