use piston_window::*;
use specs::prelude::*;
use crate::states::FinishState;
use crate::components::point::{Point, PointStyle};

pub struct RenderSystem {
  pub window: PistonWindow,
}

impl<'a> System<'a> for RenderSystem {
  type SystemData = (
    Write<'a, FinishState>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, PointStyle>,
  );

  fn run(&mut self, (mut finished, points, point_styles): Self::SystemData) {
    if let Some(event) = self.window.next() {
      self.window.draw_2d(&event, |context, graphics, _device| {
        clear([1.0; 4], graphics);

        for (point, style) in (&points, &point_styles).join() {
          if let Some(pos) = point.0 {
            ellipse(
              style.color,
              [480. + pos.x - style.radius, 360. - pos.y - style.radius, style.radius * 2., style.radius * 2.],
              context.transform,
              graphics,
            );
          }
        }

        // rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 100.0, 100.0], context.transform, graphics);
      });
    } else {
      finished.0 = true;
    }
  }
}