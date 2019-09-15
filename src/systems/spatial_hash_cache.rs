use specs::prelude::*;
use crate::{
  components::{SymbolicLine, Line, SymbolicPoint, Point},
  resources::{DirtyState, SpatialHashTable, Viewport},
};

pub struct SpatialHashCache;

impl<'a> System<'a> for SpatialHashCache {
  type SystemData = (
    Entities<'a>,
    Read<'a, DirtyState>,
    Read<'a, Viewport>,
    Write<'a, SpatialHashTable<Entity>>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, Point>,
  );

  fn run(&mut self, (entities, dirty_state, vp, mut table, sym_lines, lines, sym_points, points): Self::SystemData) {
    if dirty_state.is_solver_dirty || dirty_state.is_viewport_dirty {
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