use specs::prelude::*;
use crate::{
  events::*,
  resources::*,
  utilities::*,
  components::{symbolics::*, styles::*, markers::*},
};

pub struct InsertCircleHandler {
  command_event_reader: Option<CommandEventReader>,
}

impl Default for InsertCircleHandler {
  fn default() -> Self {
    Self { command_event_reader: None }
  }
}

impl<'a> System<'a> for InsertCircleHandler {
  type SystemData = (
    Entities<'a>,
    Read<'a, CommandEventChannel>,
    Write<'a, GeometryEventChannel>,
    Read<'a, DefaultCircleStyle>,
    WriteStorage<'a, SymbolicCircle>,
    WriteStorage<'a, CircleStyle>,
    WriteStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.command_event_reader = Some(world.fetch_mut::<CommandEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    command_event_channel,
    mut geometry_event_channel,
    default_circle_style,
    mut sym_circles,
    mut circle_styles,
    mut selecteds,
  ): Self::SystemData) {
    if let Some(reader) = &mut self.command_event_reader {
      for event in command_event_channel.read(reader) {
        match event {
          CommandEvent::InsertCircle(sym_circle) => {
            let ent = entities.create();
            let circle_style = default_circle_style.get();
            let (ent, geom) = insert(ent, *sym_circle, circle_style, &mut sym_circles, &mut circle_styles, &mut selecteds);
            geometry_event_channel.single_write(GeometryEvent::inserted(ent, geom));
          },
          CommandEvent::InsertCircleWithStyle(sym_circle, circle_style) => {
            let ent = entities.create();
            let (ent, geom) = insert(ent, *sym_circle, *circle_style, &mut sym_circles, &mut circle_styles, &mut selecteds);
            geometry_event_channel.single_write(GeometryEvent::inserted(ent, geom));
          },
          CommandEvent::InsertCircleByHistory(ent, sym_circle, circle_style) => {
            let (ent, geom) = insert(*ent, *sym_circle, *circle_style, &mut sym_circles, &mut circle_styles, &mut selecteds);
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
  sym_circle: SymbolicCircle,
  circle_style: CircleStyle,
  sym_circles: &mut WriteStorage<'a, SymbolicCircle>,
  circle_styles: &mut WriteStorage<'a, CircleStyle>,
  selecteds: &mut WriteStorage<'a, Selected>,
) -> (Entity, Geometry) {
  if let Err(err) = sym_circles.insert(ent, sym_circle) { panic!(err) }
  if let Err(err) = circle_styles.insert(ent, circle_style) { panic!(err) }
  if let Err(err) = selecteds.insert(ent, Selected) { panic!(err) }
  (ent, Geometry::Circle(sym_circle, circle_style))
}