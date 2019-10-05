use specs::prelude::*;
use crate::{
  resources::{SpatialHashTable, Viewport},
  components::{SymbolicLine, Line, SymbolicPoint, Point},
};

pub struct SpatialHashCache;

impl<'a> System<'a> for SpatialHashCache {
  type SystemData = (
    Entities<'a>,
    Read<'a, Viewport>,
    Write<'a, SpatialHashTable<Entity>>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, Point>,
  );

  fn run(&mut self, (entities, vp, mut table, sym_lines, lines, sym_points, points): Self::SystemData) {
    table.init_viewport(&*vp);
    for (ent, _, point) in (&*entities, &sym_points, &points).join() {
      table.insert_point(ent, *point, &*vp);
    }
    for (ent, _, line) in (&*entities, &sym_lines, &lines).join() {
      table.insert_line(ent, *line, &*vp);
    }
  }
}