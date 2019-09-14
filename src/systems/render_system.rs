use piston_window::*;
use specs::prelude::*;
use crate::{
  math::Vector2,
  util::Color,
  resources::{FinishState, Viewport},
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

  let (p1, p2) : (Option<Vector2>, Option<Vector2>) = if line.direction.x == 0. {
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
    if left { if p1.is_some() { p2 = Some(vec2![left_y, x_min]); } else { p1 = Some(vec2![left_y, x_min]); } }
    if right { if p1.is_some() { p2 = Some(vec2![left_y, x_max]); } else { p1 = Some(vec2![left_y, x_max]); } }

    (p1, p2)
  };

  match (p1, p2) {
    (Some(from), Some(to)) => {
      line_from_to(
        style.color.into(),
        style.width,
        vp.to_actual(from),
        vp.to_actual(to),
        context.transform,
        graphics
      );
    },
    _ => ()
  }
}

fn draw_point(point: &Point, style: &PointStyle, vp: &Viewport, context: Context, graphics: &mut G2d) {
  let Point(pos) = point;
  let actual = vp.to_actual(*pos);
  ellipse(
    style.color.into(),
    [actual[0] - style.radius, actual[1] - style.radius, style.radius * 2., style.radius * 2.],
    context.transform,
    graphics,
  );
}

pub struct RenderSystem {
  pub window: PistonWindow,
}

impl<'a> System<'a> for RenderSystem {
  type SystemData = (
    Write<'a, FinishState>,
    Read<'a, Viewport>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, PointStyle>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, LineStyle>,
  );

  fn run(&mut self, (
    mut finished,
    viewport,
    points,
    point_styles,
    lines,
    line_styles
  ): Self::SystemData) {
    if let Some(event) = self.window.next() {
      self.window.draw_2d(&event, |context, graphics, _device| {
        clear(Color::white().into(), graphics);

        // Fisrt draw lines
        for (line, style) in (&lines, &line_styles).join() {
          draw_line(line, style, &*viewport, context, graphics);
        }

        // Then draw points (as points are on top of lines)
        for (point, style) in (&points, &point_styles).join() {
          draw_point(point, style, &*viewport, context, graphics);
        }
      });
    } else {
      finished.0 = true;
    }
  }
}