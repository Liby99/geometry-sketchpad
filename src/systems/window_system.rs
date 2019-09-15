use piston_window::*;
use specs::prelude::*;
use crate::{
  math::Vector2,
  util::Color,
  resources::{FinishState, Viewport, InputState}, // , InputEvents},
  components::{Selected, Point, PointStyle, Line, LineStyle},
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

fn draw_point(point: &Point, style: &PointStyle, selected: bool, vp: &Viewport, context: Context, graphics: &mut G2d) {
  let actual = vp.to_actual(*point);
  if selected {
    let radius = style.radius + 3.0;
    circle_arc(
      Color::magenta().into(),
      1.0,
      0.0,
      std::f64::consts::PI * 1.9999,
      [actual[0] - radius, actual[1] - radius, radius * 2., radius * 2.],
      context.transform,
      graphics
    );
  }
  ellipse(
    Color::new(0.0, 0.0, 0.0, style.color.a).into(),
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
    Write<'a, Viewport>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, PointStyle>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, LineStyle>,
    ReadStorage<'a, Selected>,
  );

  fn run(&mut self, (
    mut finished,
    mut input_state,
    mut viewport,
    points,
    point_styles,
    lines,
    line_styles,
    selected,
  ): Self::SystemData) {
    input_state.reset_relative_data();
    if let Some(event) = self.window.next() {
      match event {
        Event::Input(input, _) => {
          match input {
            Input::Button(ButtonArgs { state, button, .. }) => {
              let is_pressed = state == ButtonState::Press;
              match button {
                Button::Mouse(MouseButton::Left) => input_state.mouse_left_button.set(is_pressed),
                Button::Mouse(MouseButton::Right) => input_state.mouse_right_button.set(is_pressed),
                Button::Keyboard(key) => input_state.keyboard.set(key, is_pressed),
                _ => (),
              }
            },
            Input::Move(motion) => match motion {
              Motion::MouseScroll(rel_scroll) => input_state.rel_scroll = rel_scroll,
              Motion::MouseCursor(abs_pos) => input_state.mouse_abs_pos = abs_pos,
              Motion::MouseRelative(rel_mov) => input_state.mouse_rel_movement = rel_mov,
              _ => (),
            },
            Input::Resize(ResizeArgs { window_size, .. }) => viewport.set(window_size),
            _ => (),
          }
        },
        _ => {
          self.window.draw_2d(&event, |context, graphics, _device| {
            clear(Color::white().into(), graphics); // We clean the screen

            // Fisrt draw lines
            for (line, style) in (&lines, &line_styles).join() {
              draw_line(line, style, &*viewport, context, graphics);
            }

            // Then draw regular points (not selected)
            for (point, style, _) in (&points, &point_styles, !&selected).join() {
              draw_point(point, style, false, &*viewport, context, graphics);
            }

            // Then draw selected points (as points are on top of lines)
            for (point, style, _) in (&points, &point_styles, &selected).join() {
              draw_point(point, style, true, &*viewport, context, graphics);
            }
          });
        }
      }
    } else {
      finished.0 = true;
    }
  }
}