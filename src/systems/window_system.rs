use std::time::SystemTime;
use piston_window::{Event as PistonEvent, *};
use specs::prelude::*;
use crate::{
  utilities::{Vector2, Intersect, Color, Key},
  resources::{
    DeltaTime, Viewport, ViewportTransform, InputState,
    events::{ExitEvent, ExitEventChannel, ViewportEvent, ViewportEventChannel, MouseEvent, MouseEventChannel},
  },
  components::{Hidden, Selected, Point, PointStyle, Line, LineStyle, Circle, CircleStyle, Rectangle, RectangleStyle},
};

static CLICK_TIME_THRESHOLD : u128 = 100; // 0.1 second

fn draw_line(line: &Line, style: &LineStyle, selected: bool, vp: &Viewport, context: Context, graphics: &mut G2d) {
  let aabb = vp.virtual_aabb();
  let itsct = line.intersect(aabb);
  if let Some((from, to)) = itsct {
    let from = from.to_actual(vp);
    let to = to.to_actual(vp);
    line_from_to(style.color.into(), style.width, from, to, context.transform, graphics);
    if selected {
      let Vector2 { x: dx, y: dy } = (to - from).normalized();
      let perp_dir = vec2![-dy, dx] * (style.width / 2.0 + 3.0);
      line_from_to(Color::magenta().into(), 0.5, from - perp_dir, to - perp_dir, context.transform, graphics);
      line_from_to(Color::magenta().into(), 0.5, from + perp_dir, to + perp_dir, context.transform, graphics);
    }
  }
}

fn draw_point(point: &Point, style: &PointStyle, selected: bool, vp: &Viewport, context: Context, graphics: &mut G2d) {
  let actual = point.to_actual(vp);
  if selected {
    let radius = style.radius + 3.0;
    circle_arc(
      Color::magenta().into(),
      0.5,
      0.0,
      std::f64::consts::PI * 1.9999,
      [actual.x - radius, actual.y - radius, radius * 2., radius * 2.],
      context.transform,
      graphics
    );
  }
  ellipse(
    Color::new(0.0, 0.0, 0.0, style.color.a).into(),
    [actual.x - style.radius, actual.y - style.radius, style.radius * 2., style.radius * 2.],
    context.transform,
    graphics,
  );
  let center_radius = style.radius - 1.5;
  ellipse(
    style.color.into(),
    [actual.x - center_radius, actual.y - center_radius, center_radius * 2., center_radius * 2.],
    context.transform,
    graphics,
  );
}

fn draw_circle(circle: &Circle, style: &CircleStyle, selected: bool, vp: &Viewport, context: Context, graphics: &mut G2d) {
  let actual_center = circle.center.to_actual(vp);
  let actual_radius = circle.radius.to_actual(vp);
  if selected {
    let inner_radius = actual_radius - style.width / 2.0 - 3.0;
    let outer_radius = actual_radius + style.width / 2.0 + 3.0;
    circle_arc(
      Color::magenta().into(),
      0.5,
      0.0,
      std::f64::consts::PI * 1.999999999,
      [
        actual_center.x - inner_radius,
        actual_center.y - inner_radius,
        inner_radius * 2.0,
        inner_radius * 2.0,
      ],
      context.transform,
      graphics,
    );
    circle_arc(
      Color::magenta().into(),
      0.5,
      0.0,
      std::f64::consts::PI * 1.999999999,
      [
        actual_center.x - outer_radius,
        actual_center.y - outer_radius,
        outer_radius * 2.0,
        outer_radius * 2.0,
      ],
      context.transform,
      graphics,
    )
  }
  circle_arc(
    style.color.into(),
    style.width,
    0.0,
    std::f64::consts::PI * 1.999999999,
    [actual_center.x - actual_radius, actual_center.y - actual_radius, actual_radius * 2., actual_radius * 2.],
    context.transform,
    graphics,
  );
}

fn draw_rectangle(rect: &Rectangle, style: &RectangleStyle, context: Context, graphics: &mut G2d) {
  line_from_to(style.border.color.into(), style.border.width, [rect.x, rect.y], [rect.x, rect.y + rect.height], context.transform, graphics);
  line_from_to(style.border.color.into(), style.border.width, [rect.x, rect.y], [rect.x + rect.width, rect.y], context.transform, graphics);
  line_from_to(style.border.color.into(), style.border.width, [rect.x + rect.width, rect.y], [rect.x + rect.width, rect.y + rect.height], context.transform, graphics);
  line_from_to(style.border.color.into(), style.border.width, [rect.x, rect.y + rect.height], [rect.x + rect.width, rect.y + rect.height], context.transform, graphics);
  rectangle(style.fill.into(), [rect.x, rect.y, rect.width, rect.height], context.transform, graphics);
}

pub struct WindowSystem {
  pub window: PistonWindow,
}

impl<'a> System<'a> for WindowSystem {
  type SystemData = (
    Read<'a, Viewport>,
    Write<'a, DeltaTime>,
    Write<'a, ExitEventChannel>,
    Write<'a, InputState>,
    Write<'a, ViewportEventChannel>,
    Write<'a, MouseEventChannel>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, PointStyle>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, LineStyle>,
    ReadStorage<'a, Circle>,
    ReadStorage<'a, CircleStyle>,
    ReadStorage<'a, Rectangle>,
    ReadStorage<'a, RectangleStyle>,
    ReadStorage<'a, Selected>,
    ReadStorage<'a, Hidden>,
  );

  fn run(&mut self, (
    viewport,
    mut delta_time,
    mut exit_event_channel,
    mut input_state,
    mut viewport_events,
    mut mouse_event_channel,
    points,
    point_styles,
    lines,
    line_styles,
    circles,
    circle_styles,
    rects,
    rect_styles,
    selected,
    hidden,
  ): Self::SystemData) {

    // Reset information
    input_state.reset_relative_data();

    // Handle window events
    // Will loop through and handle events until a render event happens (See line 149)
    loop {
      if let Some(event) = self.window.next() {
        match event {
          PistonEvent::Input(input, _) => {
            match input {
              Input::Button(ButtonArgs { state, button, scancode }) => {
                let is_pressed = state == ButtonState::Press;
                match button {
                  Button::Mouse(MouseButton::Left) => {
                    input_state.mouse_left_button.set(is_pressed);
                    if is_pressed {
                      input_state.mouse_left_button_last_pressed = Some(SystemTime::now());
                      mouse_event_channel.single_write(MouseEvent::MouseDown(input_state.mouse_abs_pos));
                    } else {
                      mouse_event_channel.single_write(MouseEvent::MouseUp(input_state.mouse_abs_pos));
                      if input_state.is_mouse_left_button_dragging {
                        input_state.is_mouse_left_button_dragging = false;
                        mouse_event_channel.single_write(MouseEvent::DragEnd(input_state.mouse_abs_pos));
                      } else {
                        if let Some(last_pressed) = input_state.mouse_left_button_last_pressed {
                          if last_pressed.elapsed().unwrap().as_millis() < CLICK_TIME_THRESHOLD {
                            mouse_event_channel.single_write(MouseEvent::Click(input_state.mouse_abs_pos));
                          }
                        }
                      }
                    }
                  },
                  Button::Mouse(MouseButton::Right) => {
                    input_state.mouse_right_button.set(is_pressed);
                  },
                  Button::Keyboard(piston_key) => {
                    let key = Key::from((piston_key, scancode));
                    input_state.keyboard.set(key, is_pressed)
                  },
                  _ => (),
                }
              },
              Input::Move(motion) => {
                match motion {
                  Motion::MouseScroll(rel_scroll) => input_state.rel_scroll += rel_scroll.into(),
                  Motion::MouseCursor(abs_pos) => input_state.mouse_abs_pos = abs_pos.into(),
                  Motion::MouseRelative(rel_mov) => {
                    input_state.mouse_rel_movement += rel_mov.into();
                    if input_state.is_mouse_left_button_dragging {
                      mouse_event_channel.single_write(MouseEvent::DragMove(rel_mov.into(), input_state.mouse_abs_pos));
                    } else {
                      if input_state.mouse_left_button.is_activated() {
                        input_state.is_mouse_left_button_dragging = true;
                        mouse_event_channel.single_write(MouseEvent::DragBegin(input_state.mouse_abs_pos));
                      }
                    }
                  },
                  _ => (),
                }
              },
              Input::Resize(ResizeArgs { window_size, .. }) => {
                viewport_events.single_write(ViewportEvent::Resize(Vector2::from(window_size)));
              },
              _ => (),
            }
          },
          PistonEvent::Loop(lp) => match lp {
            Loop::Update(UpdateArgs { dt }) => {
              delta_time.set(dt);
            },
            Loop::Render(_) => {
              self.window.draw_2d(&event, |context, graphics, _device| {
                clear(Color::white().into(), graphics); // We clean the screen

                // Fisrt draw regular lines
                for (line, style, _, _) in (&lines, &line_styles, !&selected, !&hidden).join() {
                  draw_line(line, style, false, &*viewport, context, graphics);
                }

                // Fisrt draw lines
                for (line, style, _, _) in (&lines, &line_styles, &selected, !&hidden).join() {
                  draw_line(line, style, true, &*viewport, context, graphics);
                }

                // Draw regular circles
                for (circle, style, _, _) in (&circles, &circle_styles, !&selected, !&hidden).join() {
                  draw_circle(circle, style, false, &*viewport, context, graphics);
                }

                // Draw selected circles
                for (circle, style, _, _) in (&circles, &circle_styles, &selected, !&hidden).join() {
                  draw_circle(circle, style, true, &*viewport, context, graphics);
                }

                // Then draw regular points (not selected)
                for (point, style, _, _) in (&points, &point_styles, !&selected, !&hidden).join() {
                  draw_point(point, style, false, &*viewport, context, graphics);
                }

                // Then draw selected points (as points are on top of lines)
                for (point, style, _, _) in (&points, &point_styles, &selected, !&hidden).join() {
                  draw_point(point, style, true, &*viewport, context, graphics);
                }

                // Draw rectangles
                for (rect, style) in (&rects, &rect_styles).join() {
                  draw_rectangle(rect, style, context, graphics);
                }
              });

              // Loop
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