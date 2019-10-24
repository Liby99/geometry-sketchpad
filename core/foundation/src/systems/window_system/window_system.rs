use piston_window::{Event as PistonEvent, *};
use specs::prelude::*;
use geopad_core_lib::{
  events::*,
  components::{screen_shapes::*, styles::*, markers::*}
};

use crate::{events::*, resources::*};
use super::{event_handling::*, rendering::*};

pub struct WindowSystem {
  pub window: PistonWindow,
}

impl<'a> System<'a> for WindowSystem {
  type SystemData = (

    // Resources
    Write<'a, ExitEventChannel>,
    Write<'a, MouseEventChannel>,
    Write<'a, ViewportEventChannel>,
    Write<'a, InputState>,
    Write<'a, DeltaTime>,

    // Data
    ReadStorage<'a, ScreenPoint>,
    ReadStorage<'a, ScreenLine>,
    ReadStorage<'a, ScreenCircle>,
    ReadStorage<'a, ScreenRectangle>,
    ReadStorage<'a, PointStyle>,
    ReadStorage<'a, LineStyle>,
    ReadStorage<'a, CircleStyle>,
    ReadStorage<'a, RectangleStyle>,
    ReadStorage<'a, Selected>,
    ReadStorage<'a, Hidden>,
  );

  fn run(&mut self, (
    mut exit_event_channel,
    mut mouse_event_channel,
    mut viewport_event_channel,
    mut input_state,
    mut delta_time,
    scrn_points,
    scrn_lines,
    scrn_circles,
    scrn_rects,
    point_styles,
    line_styles,
    circle_styles,
    rect_styles,
    selecteds,
    hiddens,
  ): Self::SystemData) {
    loop {
      if let Some(event) = self.window.next() {
        match event {
          PistonEvent::Input(input, _) => handle_input(input, &mut input_state, &mut mouse_event_channel, &mut viewport_event_channel),
          PistonEvent::Loop(lp) => match lp {
            Loop::Update(UpdateArgs { dt }) => handle_dt_update(dt, &mut delta_time),
            Loop::Render(_) => {
              render(&mut self.window, &event,
                &scrn_points, &scrn_lines, &scrn_circles, &scrn_rects,
                &point_styles, &line_styles, &circle_styles, &rect_styles,
                &selecteds, &hiddens
              );
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