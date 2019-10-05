use specs::prelude::*;
use crate::{
  util::Key,
  resources::{InputState, CreateLineData}
};

pub struct CreateLineAbortSystem;

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