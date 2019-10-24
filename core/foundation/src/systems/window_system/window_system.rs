use piston_window::{Event as PistonEvent, *};
use specs::prelude::*;
use geopad_core_lib::{events::*, math::*};

use crate::events::*;

pub struct WindowSystem {
  pub window: PistonWindow,
}

impl<'a> System<'a> for WindowSystem {
  type SystemData = (
    Write<'a, ExitEventChannel>,
    Write<'a, ViewportEventChannel>,
  );

  fn run(&mut self, (
    mut exit_event_channel,
    mut viewport_event_channel,
  ): Self::SystemData) {
    loop {
      if let Some(event) = self.window.next() {
        match event {
          PistonEvent::Input(_, _) => (), // TODO
          PistonEvent::Loop(lp) => match lp {
            Loop::Update(UpdateArgs { dt }) => {
              // delta_time.set(dt); // TODO
            },
            Loop::Render(_) => {
              self.window.draw_2d(&event, |context, graphics, _device| {
                clear(Color::white().into(), graphics); // We clean the screen
              });
              break;
            },
            _ => (),
          },
          _ => (),
        }
      } else {
        exit_event_channel.single_write(ExitEvent);
        break;
      }
    }
  }
}