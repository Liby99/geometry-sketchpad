use specs::prelude::*;
use crate::{
  components::{Line, Point},
  resources::{DirtyState, SpatialHashTable, Viewport},
};

pub struct SpatialHashCache;

impl<'a> System<'a> for SpatialHashCache {
  type SystemData = (
    Entities<'a>,
    Read<'a, DirtyState>,
    Read<'a, Viewport>,
    Write<'a, SpatialHashTable<Entity>>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, Point>,
  );

  fn run(&mut self, (entities, dirty_state, vp, mut table, lines, points): Self::SystemData) {
    if dirty_state.is_solver_dirty || dirty_state.is_viewport_dirty {
      table.init_viewport(&*vp);
      for (ent, point) in (&*entities, &points).join() {
        table.insert_point(ent, *point, &*vp);
      }
      for (ent, line) in (&*entities, &lines).join() {
        table.insert_line(ent, *line, &*vp);
      }
    }
  }
}