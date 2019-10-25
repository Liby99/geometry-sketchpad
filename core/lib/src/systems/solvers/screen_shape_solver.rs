use specs::prelude::*;
use crate::{events::*, resources::*, components::{virtual_shapes::*, screen_shapes::*}};

pub struct ScreenShapeSolver {
  viewport_event_reader: Option<ViewportEventReader>,
  geometry_event_reader: Option<GeometryEventReader>,
}

impl Default for ScreenShapeSolver {
  fn default() -> Self {
    Self {
      viewport_event_reader: None,
      geometry_event_reader: None,
    }
  }
}

impl<'a> System<'a> for ScreenShapeSolver {
  type SystemData = (
    Entities<'a>,
    Read<'a, Viewport>,
    Read<'a, DependencyGraph>,
    Read<'a, GeometryEventChannel>,
    Read<'a, ViewportEventChannel>,
    ReadStorage<'a, VirtualPoint>,
    ReadStorage<'a, VirtualLine>,
    ReadStorage<'a, VirtualCircle>,
    WriteStorage<'a, ScreenPoint>,
    WriteStorage<'a, ScreenLine>,
    WriteStorage<'a, ScreenCircle>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.viewport_event_reader = Some(world.fetch_mut::<ViewportEventChannel>().register_reader());
    self.geometry_event_reader = Some(world.fetch_mut::<GeometryEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    viewport,
    dependency_graph,
    geometry_event_channel,
    viewport_event_channel,
    virt_points,
    virt_lines,
    virt_circles,
    mut scrn_points,
    mut scrn_lines,
    mut scrn_circles,
  ): Self::SystemData) {

    // Check if there's viewport event
    let mut need_update_all = false;
    if let Some(reader) = &mut self.viewport_event_reader {
      for _ in viewport_event_channel.read(reader) {
        need_update_all = true;
        break;
      }
    }

    // Check if need update all
    if need_update_all {

      // Update all
      for (ent, virt_point) in (&entities, &virt_points).join() {
        if let Err(err) = scrn_points.insert(ent, virt_point.to_screen(&*viewport)) { panic!(err) }
      }
      for (ent, virt_line) in (&entities, &virt_lines).join() {
        if let Err(err) = scrn_lines.insert(ent, virt_line.to_screen(&*viewport)) { panic!(err) }
      }
      for (ent, virt_circle) in (&entities, &virt_circles).join() {
        if let Err(err) = scrn_circles.insert(ent, virt_circle.to_screen(&*viewport)) { panic!(err) }
      }
    } else {

      // Only update what's needed
      if let Some(reader) = &mut self.geometry_event_reader {
        for event in geometry_event_channel.read(reader) {
          match event {
            GeometryEvent::Inserted(ent, _, _) => {
              calc_scrn_shape(*ent, &viewport, &virt_points, &virt_lines, &virt_circles, &mut scrn_points, &mut scrn_lines, &mut scrn_circles);
            },
            GeometryEvent::Removed(_, _, _) => (),
            GeometryEvent::PointUpdated(ent, _, _, _) => {
              for dep in dependency_graph.get_all_dependents(ent) {
                calc_scrn_shape(dep, &viewport, &virt_points, &virt_lines, &virt_circles, &mut scrn_points, &mut scrn_lines, &mut scrn_circles);
              }
            },
            GeometryEvent::PointUpdateFinished(_, _, _, _) => ()
          }
        }
      }
    }
  }
}

fn calc_scrn_shape<'a>(
  ent: Entity,
  viewport: &Read<'a, Viewport>,
  virt_points: &ReadStorage<'a, VirtualPoint>,
  virt_lines: &ReadStorage<'a, VirtualLine>,
  virt_circles: &ReadStorage<'a, VirtualCircle>,
  scrn_points: &mut WriteStorage<'a, ScreenPoint>,
  scrn_lines: &mut WriteStorage<'a, ScreenLine>,
  scrn_circles: &mut WriteStorage<'a, ScreenCircle>,
) {
  if let Some(virt_point) = virt_points.get(ent) {
    if let Err(err) = scrn_points.insert(ent, virt_point.to_screen(&*viewport)) { panic!(err) }
  } else if let Some(virt_line) = virt_lines.get(ent) {
    if let Err(err) = scrn_lines.insert(ent, virt_line.to_screen(&*viewport)) { panic!(err) }
  } else if let Some(virt_circle) = virt_circles.get(ent) {
    if let Err(err) = scrn_circles.insert(ent, virt_circle.to_screen(&*viewport)) { panic!(err) }
  }
}