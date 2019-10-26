use specs::prelude::*;
use core_ui::{events::*, resources::*};
use super::input::UserEvent;

pub struct ReceiverSystem {
  pub receiver: std::sync::mpsc::Receiver<UserEvent>,
}

impl<'a> System<'a> for ReceiverSystem {
  type SystemData = (
    Write<'a, DeltaTime>,
    Write<'a, ExitEventChannel>,
  );

  fn run(&mut self, (
    mut delta_time,
    mut exit_event_channel,
  ): Self::SystemData) {
    loop {
      match self.receiver.try_recv() {
        Ok(user_event) => match user_event {
          UserEvent::Loop(dt) => {
            delta_time.set(dt);
            break;
          },
          UserEvent::Input(_input) => {
            // TODO
          },
          UserEvent::Shutdown => {
            exit_event_channel.single_write(ExitEvent);
          },
        },
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
          exit_event_channel.single_write(ExitEvent);
        },
        Err(std::sync::mpsc::TryRecvError::Empty) => (),
      }
    }
  }
}