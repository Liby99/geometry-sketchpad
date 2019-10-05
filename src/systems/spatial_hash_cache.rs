use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::{
  resources::{SpatialHashTable, Viewport, ViewportEvent},
  components::{SymbolicLine, Line, SymbolicPoint, Point},
};

pub struct SpatialHashCache {
  viewport_events_reader_id: Option<ReaderId<ViewportEvent>>,
}

impl Default for SpatialHashCache {
  fn default() -> Self {
    Self {
      viewport_events_reader_id: None
    }
  }
}

impl SpatialHashCache {
  fn need_refresh(&mut self, vp_events: &EventChannel<ViewportEvent>) -> bool {
    if let Some(vp_event_reader_id) = &mut self.viewport_events_reader_id {
      for _ in vp_events.read(vp_event_reader_id) {
        return true;
      }
    }
    return false;
  }
}

impl<'a> System<'a> for SpatialHashCache {
  type SystemData = (
    Entities<'a>,
    Read<'a, Viewport>,
    Read<'a, EventChannel<ViewportEvent>>,
    Write<'a, SpatialHashTable<Entity>>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, Point>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);

    // Setup the reader id
    self.viewport_events_reader_id = Some(world.fetch_mut::<EventChannel<ViewportEvent>>().register_reader());
  }

  fn run(&mut self, (
    entities,
    vp,
    vp_events,
    mut table,
    sym_lines,
    lines,
    sym_points,
    points
  ): Self::SystemData) {
    if self.need_refresh(&*vp_events) {
      table.init_viewport(&*vp);
      for (ent, _, point) in (&*entities, &sym_points, &points).join() {
        table.insert_point(ent, *point, &*vp);
      }
      for (ent, _, line) in (&*entities, &sym_lines, &lines).join() {
        table.insert_line(ent, *line, &*vp);
      }
    }
  }
}