use specs::prelude::*;
use crate::{
  events::*,
  resources::*,
  utilities::*,
  components::{symbolics::*, styles::*, markers::*},
};

pub struct InsertLineHandler {
  command_event_reader: Option<CommandEventReader>,
}

impl Default for InsertLineHandler {
  fn default() -> Self {
    Self { command_event_reader: None }
  }
}

impl<'a> System<'a> for InsertLineHandler {
  type SystemData = (
    Entities<'a>,
    Read<'a, CommandEventChannel>,
    Write<'a, GeometryEventChannel>,
    Read<'a, DefaultLineStyle>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, Selected>,
    WriteStorage<'a, Element>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.command_event_reader = Some(world.fetch_mut::<CommandEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    command_event_channel,
    mut geometry_event_channel,
    default_line_style,
    mut sym_lines,
    mut line_styles,
    mut selecteds,
    mut elements,
  ): Self::SystemData) {
    if let Some(reader) = &mut self.command_event_reader {
      for event in command_event_channel.read(reader) {
        match event {
          CommandEvent::LineInsert(insert_line_event) => match insert_line_event {
            InsertLineEvent::InsertLine(sym_line) => {
              let ent = entities.create();
              let line_style = default_line_style.get();
              let (ent, geom) = insert(ent, *sym_line, line_style, &mut sym_lines, &mut line_styles, &mut selecteds, &mut elements);
              geometry_event_channel.single_write(GeometryEvent::inserted(ent, geom));
            },
            InsertLineEvent::InsertLineWithStyle(sym_line, line_style) => {
              let ent = entities.create();
              let (ent, geom) = insert(ent, *sym_line, *line_style, &mut sym_lines, &mut line_styles, &mut selecteds, &mut elements);
              geometry_event_channel.single_write(GeometryEvent::inserted(ent, geom));
            },
            InsertLineEvent::InsertLineByHistory(ent, sym_line, line_style) => {
              let (ent, geom) = insert(*ent, *sym_line, *line_style, &mut sym_lines, &mut line_styles, &mut selecteds, &mut elements);
              geometry_event_channel.single_write(GeometryEvent::inserted_by_history(ent, geom));
            },
          },
          _ => (),
        }
      }
    }
  }
}

fn insert<'a>(
  ent: Entity,
  sym_line: SymbolicLine,
  line_style: LineStyle,
  sym_lines: &mut WriteStorage<'a, SymbolicLine>,
  line_styles: &mut WriteStorage<'a, LineStyle>,
  selecteds: &mut WriteStorage<'a, Selected>,
  elements: &mut WriteStorage<'a, Element>,
) -> (Entity, Geometry) {
  if let Err(err) = sym_lines.insert(ent, sym_line) { panic!(err) }
  if let Err(err) = line_styles.insert(ent, line_style) { panic!(err) }
  if let Err(err) = selecteds.insert(ent, Selected) { panic!(err) }
  if let Err(err) = elements.insert(ent, Element) { panic!(err) }
  (ent, Geometry::Line(sym_line, line_style))
}