use specs::prelude::*;
use crate::{
  components::{SymbolicLine, Line, SymbolicPoint, Point},
  resources::{SpatialHashTable, Events, Viewport},
};

pub struct SpatialHashCache;

impl<'a> System<'a> for SpatialHashCache {
  type SystemData = (
    Entities<'a>,
    Read<'a, Viewport>,
    Read<'a, Events>,
    Write<'a, SpatialHashTable<Entity>>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, Point>,
  );

  fn run(&mut self, (entities, vp, events, mut table, sym_lines, lines, sym_points, points): Self::SystemData) {
    if events.has_viewport_event() {
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