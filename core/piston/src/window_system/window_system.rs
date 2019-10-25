use piston_window::{Event as PistonEvent, *};
use specs::prelude::*;
use core_lib::{events::*, resources::Viewport, components::{screen_shapes::*, styles::*, markers::*}};
use core_ui::{events::*, resources::*};

use super::{event_handling::*, rendering::*};

pub struct WindowSystem {
  pub window: PistonWindow,
}

impl<'a> System<'a> for WindowSystem {
  type SystemData = (

    // Resources
    Read<'a, Viewport>,
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
    viewport,
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
    input_state.reset_relative_data();
    loop {
      if let Some(event) = self.window.next() {
        match event {
          PistonEvent::Input(input, _) => handle_input(input, &mut input_state, &mut mouse_event_channel, &mut viewport_event_channel),
          PistonEvent::Loop(lp) => match lp {
            Loop::Update(UpdateArgs { dt }) => handle_dt_update(dt, &mut delta_time),
            Loop::Render(_) => {
              render(&mut self.window, &event, &*viewport,
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