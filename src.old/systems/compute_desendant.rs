use specs::prelude::*;
use crate::{
  resources::{DirtyState, DescendantMap},
  components::{SymbolicLine, SymbolicPoint},
};

pub struct ComputeDescendant;

impl<'a> System<'a> for ComputeDescendant {
  type SystemData = (
    Read<'a, DirtyState>,
    Write<'a, DescendantMap>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, SymbolicPoint>,
  );

  fn run(&mut self, (dirty_state, mut descendant_map, sym_lines, sym_points): Self::SystemData) {
    if dirty_state.is_sym_elem_dirty {
      // Do nothing now
    }
  }
}