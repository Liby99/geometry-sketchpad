use specs::prelude::*;
use crate::{
  resources::events::{ExitEventChannel, ExitEventReader},
};

pub struct ExitState(bool);

impl Default for ExitState {
  fn default() -> Self {
    Self(false)
  }
}

impl ExitState {
  pub fn set_need_exit(&mut self) {
    self.0 = true;
  }

  pub fn is_running(&self) -> bool {
    !self.0
  }
}

pub struct ExitStateManager {
  exit_event_reader_id: Option<ExitEventReader>,
}

impl Default for ExitStateManager {
  fn default() -> Self {
    Self { exit_event_reader_id: None }
  }
}

impl<'a> System<'a> for ExitStateManager {
  type SystemData = (
    Read<'a, ExitEventChannel>,
    Write<'a, ExitState>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.exit_event_reader_id = Some(world.fetch_mut::<ExitEventChannel>().register_reader());
  }

  fn run(&mut self, (exit_event_channel, mut exit_state): Self::SystemData) {
    if let Some(reader_id) = &mut self.exit_event_reader_id {
      for _ in exit_event_channel.read(reader_id) {
        exit_state.set_need_exit();
        break;
      }
    } else {
      panic!("[exit_state_manager] No reader id");
    }
  }
}