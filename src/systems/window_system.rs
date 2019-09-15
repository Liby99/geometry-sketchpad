use piston_window::*;
use specs::prelude::*;
use crate::{
  math::Vector2,
  util::Color,
  resources::{FinishState, Viewport, InputState}, // , InputEvents},
  components::{
    point::{Point, PointStyle},
    line::{Line, LineStyle},
  }
};

fn draw_line(line: &Line, style: &LineStyle, vp: &Viewport, context: Context, graphics: &mut G2d) {
  let x_min = vp.x_min();
  let x_max = vp.x_max();
  let y_min = vp.y_min();
  let y_max = vp.y_max();

  if let (Some(from), Some(to)) = if line.direction.x == 0. {
    if line.origin.y >= y_min && line.origin.y <= y_max {
      (Some(vec2![line.origin.x, y_min]), Some(vec2![line.origin.x, y_max]))
    } else {
      (None, None)
    }
  } else if line.direction.y == 0. {
    if line.origin.y >= y_min && line.origin.y <= y_max {
      (Some(vec2![x_min, line.origin.y]), Some(vec2![x_max, line.origin.y]))
    } else {
      (None, None)
    }
  } else {
    let bottom_x = line.origin.x + (y_min - line.origin.y) / line.direction.y * line.direction.x;
    let top_x = line.origin.x + (y_max - line.origin.y) / line.direction.y * line.direction.x;
    let left_y = line.origin.y + (x_min - line.origin.x) / line.direction.x * line.direction.y;
    let right_y = line.origin.y + (x_max - line.origin.x) / line.direction.x * line.direction.y;

    let bottom = bottom_x >= x_min && bottom_x <= x_max;
    let top = top_x >= x_min && top_x <= x_max;
    let left = left_y >= y_min && left_y <= y_max;
    let right = right_y >= y_min && right_y <= y_max;

    let mut p1 = None;
    let mut p2 = None;

    if bottom { p1 = Some(vec2![bottom_x, y_min]); }
    if top { if p1.is_some() { p2 = Some(vec2![top_x, y_max]); } else { p1 = Some(vec2![top_x, y_max]); } }
    if left { if p1.is_some() { p2 = Some(vec2![x_min, left_y]); } else { p1 = Some(vec2![x_min, left_y]); } }
    if right { if p1.is_some() { p2 = Some(vec2![x_max, right_y]); } else { p1 = Some(vec2![x_max, right_y]); } }

    (p1, p2)
  } {
    let from = vp.to_actual(from);
    let to = vp.to_actual(to);
    line_from_to(style.color.into(), style.width, from, to, context.transform, graphics);
  }
}

fn draw_point(point: &Point, style: &PointStyle, vp: &Viewport, context: Context, graphics: &mut G2d) {
  let Point(pos) = point;
  let actual = vp.to_actual(*pos);
  ellipse(
    Color::black().into(),
    [actual[0] - style.radius, actual[1] - style.radius, style.radius * 2., style.radius * 2.],
    context.transform,
    graphics,
  );
  let center_radius = style.radius - 1.5;
  ellipse(
    style.color.into(),
    [actual[0] - center_radius, actual[1] - center_radius, center_radius * 2., center_radius * 2.],
    context.transform,
    graphics,
  );
}

pub struct WindowSystem {
  pub window: PistonWindow,
}

impl<'a> System<'a> for WindowSystem {
  type SystemData = (
    Write<'a, FinishState>,
    Write<'a, InputState>,
    Read<'a, Viewport>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, PointStyle>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, LineStyle>,
  );

  fn run(&mut self, (
    mut finished,
    mut mouse_state,
    viewport,
    points,
    point_styles,
    lines,
    line_styles
  ): Self::SystemData) {
    mouse_state.reset_relative_data();
    if let Some(event) = self.window.next() {
      match event {
        Event::Input(input, _) => {
          match input {
            Input::Button(ButtonArgs { state, button, .. }) => {
              let is_pressed = state == ButtonState::Press;
              match button {
                Button::Mouse(MouseButton::Left) => mouse_state.left_button.set(is_pressed),
                _ => ()
              }
            },
            Input::Move(motion) => match motion {
              Motion::MouseScroll(rel_scroll) => mouse_state.rel_scroll = rel_scroll,
              _ => ()
            },
            _ => ()
          }
        },
        _ => {
          self.window.draw_2d(&event, |context, graphics, _device| {
            // input_events.clear(); // Every time we do render, we clear the input events
            clear(Color::white().into(), graphics); // We clean the screen

            // Fisrt draw lines
            for (line, style) in (&lines, &line_styles).join() {
              draw_line(line, style, &*viewport, context, graphics);
            }

            // Then draw points (as points are on top of lines)
            for (point, style) in (&points, &point_styles).join() {
              draw_point(point, style, &*viewport, context, graphics);
            }
          });
        }
      }
    } else {
      finished.0 = true;
    }
  }
}