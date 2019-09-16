use piston_window::*;
use specs::prelude::*;
use crate::{
  math::Intersect,
  util::Color,
  resources::{FinishState, Viewport, ViewportTransform, InputState, DirtyState},
  components::{Selected, Point, PointStyle, Line, LineStyle},
};

fn draw_line(line: &Line, style: &LineStyle, vp: &Viewport, context: Context, graphics: &mut G2d) {
  let aabb = vp.virtual_aabb();
  let itsct = line.intersect(aabb);
  if let Some((from, to)) = itsct {
    let from = from.to_actual(vp);
    let to = to.to_actual(vp);
    line_from_to(style.color.into(), style.width, from, to, context.transform, graphics);
  }
}

fn draw_point(point: &Point, style: &PointStyle, selected: bool, vp: &Viewport, context: Context, graphics: &mut G2d) {
  let actual = point.to_actual(vp);
  if selected {
    let radius = style.radius + 3.0;
    circle_arc(
      Color::magenta().into(),
      1.0,
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

pub struct WindowSystem {
  pub window: PistonWindow,
}

impl<'a> System<'a> for WindowSystem {
  type SystemData = (
    Write<'a, FinishState>,
    Write<'a, InputState>,
    Write<'a, DirtyState>,
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
    mut dirty_state,
    mut viewport,
    points,
    point_styles,
    lines,
    line_styles,
    selected,
  ): Self::SystemData) {

    // Reset information
    dirty_state.reset();
    input_state.reset_relative_data();

    // Handle window events
    if let Some(event) = self.window.next() {
      match event {
        Event::Input(input, _) => {

          // Set the input state dirty
          dirty_state.is_input_dirty = true;
          match input {
            Input::Button(ButtonArgs { state, button, .. }) => {
              let is_pressed = state == ButtonState::Press;
              match button {
                Button::Mouse(MouseButton::Left) => input_state.mouse_left_button.set(is_pressed),
                Button::Mouse(MouseButton::Right) => input_state.mouse_right_button.set(is_pressed),
                Button::Keyboard(key) => input_state.keyboard.set(key, is_pressed),
                _ => dirty_state.is_input_dirty = false,
              }
            },
            Input::Move(motion) => match motion {
              Motion::MouseScroll(rel_scroll) => input_state.rel_scroll = rel_scroll.into(),
              Motion::MouseCursor(abs_pos) => input_state.mouse_abs_pos = abs_pos.into(),
              Motion::MouseRelative(rel_mov) => input_state.mouse_rel_movement = rel_mov.into(),
              _ => dirty_state.is_input_dirty = false,
            },
            Input::Resize(ResizeArgs { window_size, .. }) => {
              viewport.set(window_size);
              dirty_state.is_viewport_dirty = true;
            },
            _ => dirty_state.is_input_dirty = false,
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