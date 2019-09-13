use piston_window::*;
use specs::prelude::*;
use crate::states::FinishState;
use crate::util::Color;
use crate::components::{
  point::{Point, PointStyle},
  line::{Line, LineStyle},
};

pub struct RenderSystem {
  pub window: PistonWindow,
}

impl<'a> System<'a> for RenderSystem {
  type SystemData = (
    Write<'a, FinishState>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, PointStyle>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, LineStyle>,
  );

  fn run(&mut self, (
    mut finished,
    points,
    point_styles,
    lines,
    line_styles
  ): Self::SystemData) {
    if let Some(event) = self.window.next() {
      self.window.draw_2d(&event, |context, graphics, _device| {
        clear(Color::white().into(), graphics);

        let x_min = -480.;
        let x_max = -x_min;
        let y_min = -360.;
        let y_max = -y_min;

        for (line, style) in (&lines, &line_styles).join() {

          let (p1, p2) = if line.direction.x == 0. {
            if line.origin.y >= y_min && line.origin.y <= y_max {
              (Some([line.origin.x, y_min]), Some([line.origin.x, y_max]))
            } else {
              (None, None)
            }
          } else if line.direction.y == 0. {
            if line.origin.y >= y_min && line.origin.y <= y_max {
              (Some([x_min, line.origin.y]), Some([x_max, line.origin.y]))
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

            if bottom { p1 = Some([bottom_x, y_min]); }
            if top { if p1.is_some() { p2 = Some([top_x, y_max]); } else { p1 = Some([top_x, y_max]); } }
            if left { if p1.is_some() { p2 = Some([left_y, x_min]); } else { p1 = Some([left_y, x_min]); } }
            if right { if p1.is_some() { p2 = Some([left_y, x_max]); } else { p1 = Some([left_y, x_max]); } }

            (p1, p2)
          };

          match (p1, p2) {
            (Some(from), Some(to)) => {
              line_from_to(
                style.color.into(),
                style.width,
                [480. + from[0], 360. - from[1]],
                [480. + to[0], 360. - to[1]],
                context.transform,
                graphics,
              );
            },
            _ => ()
          }
        }

        // Draw points
        for (point, style) in (&points, &point_styles).join() {
          let pos = point.0;
          ellipse(
            style.color.into(),
            [480. + pos.x - style.radius, 360. - pos.y - style.radius, style.radius * 2., style.radius * 2.],
            context.transform,
            graphics,
          );
        }
      });
    } else {
      finished.0 = true;
    }
  }
}