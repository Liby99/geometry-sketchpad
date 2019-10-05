use specs::prelude::*;
use crate::{
  util::Key,
  resources::{InputState, CreateLineData}
};

pub struct CreateLineAbortSystem;

/// # Create Line Abort System
///
/// This system intended to provide mechanism to abort a line creation process
/// When a user already place the first point of the line, if the user
/// regret this decision, they can press `Escape` to abort this line creation
/// process.
///
/// TODO: If a new point is created during this line creation process, we should
/// remove that new point as well when aborting (for better ergonomics)
impl<'a> System<'a> for CreateLineAbortSystem {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, CreateLineData>,
  );

  fn run(&mut self, (input_state, mut create_line_data): Self::SystemData) {
    if input_state.keyboard.just_activated(Key::Escape) {
      if create_line_data.maybe_first_point.is_some() {
        create_line_data.maybe_first_point = None;
      }
    }
  }
}