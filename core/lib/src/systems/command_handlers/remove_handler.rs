use std::collections::HashSet;
use specs::prelude::*;
use crate::{
  events::*,
  utilities::*,
  components::{symbolics::*, styles::*, virtual_shapes::*, screen_shapes::*, markers::*},
};

pub struct RemoveHandler {
  command_event_reader: Option<CommandEventReader>,
}

impl Default for RemoveHandler {
  fn default() -> Self {
    Self { command_event_reader: None }
  }
}

impl<'a> System<'a> for RemoveHandler {
  type SystemData = (
    Entities<'a>,
    Read<'a, CommandEventChannel>,
    Write<'a, GeometryEventChannel>,

    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, VirtualPoint>,
    WriteStorage<'a, ScreenPoint>,

    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, VirtualLine>,
    WriteStorage<'a, ScreenLine>,

    WriteStorage<'a, SymbolicCircle>,
    WriteStorage<'a, CircleStyle>,
    WriteStorage<'a, VirtualCircle>,
    WriteStorage<'a, ScreenCircle>,

    WriteStorage<'a, Element>,
    WriteStorage<'a, Selected>,
    WriteStorage<'a, Hidden>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.command_event_reader = Some(world.fetch_mut::<CommandEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    command_event_channel,
    mut geometry_event_channel,

    mut sym_points,
    mut point_styles,
    mut virt_points,
    mut scrn_points,

    mut sym_lines,
    mut line_styles,
    mut virt_lines,
    mut scrn_lines,

    mut sym_circles,
    mut circle_styles,
    mut virt_circles,
    mut scrn_circles,

    mut elements,
    mut selecteds,
    mut hiddens,
  ): Self::SystemData) {
    if let Some(reader) = &mut self.command_event_reader {
      for event in command_event_channel.read(reader) {
        match event {
          CommandEvent::Remove(remove_event) => match remove_event {
            RemoveEvent::Remove(ent) => {
              let geom = remove_element(ent, &mut sym_points, &mut point_styles, &mut virt_points, &mut scrn_points, &mut sym_lines, &mut line_styles, &mut virt_lines, &mut scrn_lines, &mut sym_circles, &mut circle_styles, &mut virt_circles, &mut scrn_circles, &mut elements, &mut selecteds, &mut hiddens);
              geometry_event_channel.single_write(GeometryEvent::removed(*ent, geom));
            },
            RemoveEvent::RemoveByHistory(ent) => {
              let geom = remove_element(ent, &mut sym_points, &mut point_styles, &mut virt_points, &mut scrn_points, &mut sym_lines, &mut line_styles, &mut virt_lines, &mut scrn_lines, &mut sym_circles, &mut circle_styles, &mut virt_circles, &mut scrn_circles, &mut elements, &mut selecteds, &mut hiddens);
              geometry_event_channel.single_write(GeometryEvent::removed_by_history(*ent, geom));
            },
            RemoveEvent::RemoveSelected => {
              let mut set = HashSet::new();
              for (ent, _) in (&entities, &selecteds).join() {
                set.insert(ent);
              }
              for ent in set {
                let geom = remove_element(&ent, &mut sym_points, &mut point_styles, &mut virt_points, &mut scrn_points, &mut sym_lines, &mut line_styles, &mut virt_lines, &mut scrn_lines, &mut sym_circles, &mut circle_styles, &mut virt_circles, &mut scrn_circles, &mut elements, &mut selecteds, &mut hiddens);
                geometry_event_channel.single_write(GeometryEvent::removed(ent, geom));
              }
            },
            RemoveEvent::RemoveAll => {
              let mut set = HashSet::new();
              for (ent, _) in (&entities, &elements).join() {
                set.insert(ent);
              }
              for ent in set {
                let geom = remove_element(&ent, &mut sym_points, &mut point_styles, &mut virt_points, &mut scrn_points, &mut sym_lines, &mut line_styles, &mut virt_lines, &mut scrn_lines, &mut sym_circles, &mut circle_styles, &mut virt_circles, &mut scrn_circles, &mut elements, &mut selecteds, &mut hiddens);
                geometry_event_channel.single_write(GeometryEvent::removed(ent, geom));
              }
            },
          },
          _ => (), // Don't care others
        }
      }
    }
  }
}

fn remove_element<'a>(
  ent: &Entity,

  sym_points: &mut WriteStorage<'a, SymbolicPoint>,
  point_styles: &mut WriteStorage<'a, PointStyle>,
  virt_points: &mut WriteStorage<'a, VirtualPoint>,
  scrn_points: &mut WriteStorage<'a, ScreenPoint>,

  sym_lines: &mut WriteStorage<'a, SymbolicLine>,
  line_styles: &mut WriteStorage<'a, LineStyle>,
  virt_lines: &mut WriteStorage<'a, VirtualLine>,
  scrn_lines: &mut WriteStorage<'a, ScreenLine>,

  sym_circles: &mut WriteStorage<'a, SymbolicCircle>,
  circle_styles: &mut WriteStorage<'a, CircleStyle>,
  virt_circles: &mut WriteStorage<'a, VirtualCircle>,
  scrn_circles: &mut WriteStorage<'a, ScreenCircle>,

  elements: &mut WriteStorage<'a, Element>,
  selecteds: &mut WriteStorage<'a, Selected>,
  hiddens: &mut WriteStorage<'a, Hidden>,
) -> Geometry {

  // Remove from markers
  elements.remove(*ent);
  selecteds.remove(*ent);
  hiddens.remove(*ent);

  // Remove from geometry storages
  if let Some(sym_point) = sym_points.remove(*ent) {
    if let Some(point_style) = point_styles.remove(*ent) {
      virt_points.remove(*ent);
      scrn_points.remove(*ent);
      Geometry::Point(sym_point, point_style)
    } else {
      panic!("Entity to remove does not exist: {:?}", ent);
    }
  } else if let Some(sym_line) = sym_lines.remove(*ent) {
    if let Some(line_style) = line_styles.remove(*ent) {
      virt_lines.remove(*ent);
      scrn_lines.remove(*ent);
      Geometry::Line(sym_line, line_style)
    } else {
      panic!("Entity to remove does not exist: {:?}", ent);
    }
  } else if let Some(sym_circle) = sym_circles.remove(*ent) {
    if let Some(circle_style) = circle_styles.remove(*ent) {
      virt_circles.remove(*ent);
      scrn_circles.remove(*ent);
      Geometry::Circle(sym_circle, circle_style)
    } else {
      panic!("Entity to remove does not exist: {:?}", ent);
    }
  } else {
    panic!("Entity to remove does not exist: {:?}", ent);
  }
}