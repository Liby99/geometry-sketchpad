use piston_window::*;
use specs::prelude::*;
use crate::states::FinishState;

pub struct RenderSystem {
  pub window: PistonWindow,
}

impl<'a> System<'a> for RenderSystem {
  type SystemData = Write<'a, FinishState>;

  fn run(&mut self, mut finished: Self::SystemData) {
    if let Some(event) = self.window.next() {
      self.window.draw_2d(&event, |context, graphics, _device| {
        clear([1.0; 4], graphics);
        rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 100.0, 100.0], context.transform, graphics);
      });
    } else {
      finished.0 = true;
    }
  }
}