use specs::prelude::*;
use crate::{
  events::*,
  resources::*,
  utilities::*,
  components::{symbolics::*, styles::*, markers::*},
};

pub struct InsertPointHandler {
  command_event_reader: Option<CommandEventReader>,
}

impl Default for InsertPointHandler {
  fn default() -> Self {
    Self { command_event_reader: None }
  }
}

impl<'a> System<'a> for InsertPointHandler {
  type SystemData = (
    Entities<'a>,
    Read<'a, CommandEventChannel>,
    Write<'a, GeometryEventChannel>,
    Read<'a, DefaultPointStyle>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, PointStyle>,
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
    default_point_style,
    mut sym_points,
    mut point_styles,
    mut selecteds,
    mut elements,
  ): Self::SystemData) {
    if let Some(reader) = &mut self.command_event_reader {
      for event in command_event_channel.read(reader) {
        match event {
          CommandEvent::InsertPoint(sym_point) => {
            let ent = entities.create();
            let point_style = default_point_style.get();
            let (ent, geom) = insert(ent, *sym_point, point_style, &mut sym_points, &mut point_styles, &mut selecteds, &mut elements);
            geometry_event_channel.single_write(GeometryEvent::inserted(ent, geom));
          },
          CommandEvent::InsertPointWithStyle(sym_point, point_style) => {
            let ent = entities.create();
            let (ent, geom) = insert(ent, *sym_point, *point_style, &mut sym_points, &mut point_styles, &mut selecteds, &mut elements);
            geometry_event_channel.single_write(GeometryEvent::inserted(ent, geom));
          },
          CommandEvent::InsertPointByHistory(ent, sym_point, point_style) => {
            let (ent, geom) = insert(*ent, *sym_point, *point_style, &mut sym_points, &mut point_styles, &mut selecteds, &mut elements);
            geometry_event_channel.single_write(GeometryEvent::inserted_by_history(ent, geom));
          },
          _ => (),
        }
      }
    }
  }
}

fn insert<'a>(
  ent: Entity,
  sym_point: SymbolicPoint,
  point_style: PointStyle,
  sym_points: &mut WriteStorage<'a, SymbolicPoint>,
  point_styles: &mut WriteStorage<'a, PointStyle>,
  selecteds: &mut WriteStorage<'a, Selected>,
  elements: &mut WriteStorage<'a, Element>,
) -> (Entity, Geometry) {
  if let Err(err) = sym_points.insert(ent, sym_point) { panic!(err) }
  if let Err(err) = point_styles.insert(ent, point_style) { panic!(err) }
  if let Err(err) = selecteds.insert(ent, Selected) { panic!(err) }
  if let Err(err) = elements.insert(ent, Element) { panic!(err) }
  (ent, Geometry::Point(sym_point, point_style))
}