use specs::prelude::*;
use crate::{
  resources::{SpatialHashTable, Viewport},
  components::{SymbolicLine, Line, SymbolicPoint, Point},
  systems::events::{ViewportEventChannel, ViewportEventReader, Geometry, SketchEvent, SketchEventChannel, SketchEventReader},
};

pub struct SpatialHashCache {
  viewport_events_reader_id: Option<ViewportEventReader>,
  sketch_events_reader_id: Option<SketchEventReader>,
}

impl Default for SpatialHashCache {
  fn default() -> Self {
    Self {
      viewport_events_reader_id: None,
      sketch_events_reader_id: None,
    }
  }
}

impl SpatialHashCache {
  fn need_refresh(&mut self, vp_events: &ViewportEventChannel) -> bool {
    if let Some(vp_event_reader_id) = &mut self.viewport_events_reader_id {
      for _ in vp_events.read(vp_event_reader_id) {
        return true;
      }
      return false;
    } else {
      panic!("[spatial_hash_cache] No viewport event reader id");
    }
  }
}

impl<'a> System<'a> for SpatialHashCache {
  type SystemData = (
    Entities<'a>,
    Read<'a, Viewport>,
    Read<'a, ViewportEventChannel>,
    Read<'a, SketchEventChannel>,
    Write<'a, SpatialHashTable<Entity>>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, Point>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);

    // Setup the reader id
    self.viewport_events_reader_id = Some(world.fetch_mut::<ViewportEventChannel>().register_reader());
    self.sketch_events_reader_id = Some(world.fetch_mut::<SketchEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    vp,
    viewport_event_channel,
    sketch_events,
    mut table,
    sym_lines,
    lines,
    sym_points,
    points
  ): Self::SystemData) {

    // First check if needs full refresh
    if self.need_refresh(&*viewport_event_channel) {

      // If is then reconstruct the whole table
      table.init_viewport(&*vp);
      for (ent, _, point) in (&*entities, &sym_points, &points).join() {
        table.insert_point(ent, *point, &*vp);
      }
      for (ent, _, line) in (&*entities, &sym_lines, &lines).join() {
        table.insert_line(ent, *line, &*vp);
      }
    } else {

      // Else, loop through all the events
      if let Some(sketch_event_reader_id) = &mut self.sketch_events_reader_id {
        for event in sketch_events.read(sketch_event_reader_id) {
          match event {
            SketchEvent::Insert(entity, geom) => match geom {
              Geometry::Point(_, _) => match points.get(*entity) {
                Some(position) => table.insert_point(*entity, *position, &*vp),
                None => panic!("[spatial_hash_cache] Cannot find given point"),
              },
              Geometry::Line(_, _) => match lines.get(*entity) {
                Some(line) => table.insert_line(*entity, *line, &*vp),
                None => panic!("[spatial_hash_cache] Cannot find given line"),
              }
            },
            SketchEvent::Remove(entity, _) => table.remove_from_all(*entity),
            SketchEvent::Select(_) | SketchEvent::Deselect(_) => (),
          }
        }
      } else {
        panic!("[spatial_hash_cache] No sketch event reader id");
      }
    }
  }
}