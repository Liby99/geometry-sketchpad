use crate::{events::*, resources::*};
use specs::prelude::*;

pub struct ExitStateManager {
    exit_event_reader: Option<ExitEventReader>,
}

impl Default for ExitStateManager {
    fn default() -> Self {
        Self {
            exit_event_reader: None,
        }
    }
}

impl<'a> System<'a> for ExitStateManager {
    type SystemData = (Read<'a, ExitEventChannel>, Write<'a, ExitState>);

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.exit_event_reader = Some(world.fetch_mut::<ExitEventChannel>().register_reader());
    }

    fn run(&mut self, (exit_event_channel, mut exit_state): Self::SystemData) {
        if let Some(reader) = &mut self.exit_event_reader {
            for _ in exit_event_channel.read(reader) {
                exit_state.set_need_exit();
                break;
            }
        } else {
            panic!("[exit_state_manager] No reader id");
        }
    }
}
