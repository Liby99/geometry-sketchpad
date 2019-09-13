extern crate piston_window;

mod geometry;
mod storage;
mod math;
mod context;

use piston_window::*;

use math::Vec2;
use geometry::{
  point::{ Point, PointConstruct },
  line::{ Line, LineConstruct },
};

impl Into<types::Vec2d> for Vec2 {
  fn into(self) -> types::Vec2d {
    [self.x, self.y]
  }
}

fn main() -> Result<(), context::SolveError> {

  let mut context = context::Context::new();

  let pa = context.points.add(PointConstruct::Free { pos: Vec2::new(-20., 0.) });
  let pb = context.points.add(PointConstruct::Free { pos: Vec2::new(-10., 10.) });
  let pc = context.points.add(PointConstruct::Free { pos: Vec2::new(10., 10.) });
  let pd = context.points.add(PointConstruct::Free { pos: Vec2::new(20., 0.) });
  let l1 = context.lines.add(LineConstruct::TwoPoint { p1: pa, p2: pb });
  let l2 = context.lines.add(LineConstruct::TwoPoint { p1: pd, p2: pc });
  let x_axis = context.lines.add(LineConstruct::TwoPoint { p1: pa, p2: pd });
  let pe = context.points.add(PointConstruct::LineLineIntersect { l1, l2 });
  let l3 = context.lines.add(LineConstruct::Parallel { l: x_axis, p: pe });

  let solution = context.solve()?;

  let offset = Vec2 { x: 320., y: 240. };

  let mut window : PistonWindow = WindowSettings::new("Geopad!", [640, 480]).exit_on_esc(true).build().unwrap();
  while let Some(event) = window.next() {
    window.draw_2d(&event, |context, graphics, _device| {
      clear([1.0; 4], graphics);
      for (_, line) in solution.clone().lines {
        line_from_to(
          [0.0, 0.0, 0.0, 1.0],
          3.0,
          offset + line.origin - 100.0 * line.direction, // from
          offset + line.origin + 100.0 * line.direction, // to
          context.transform,
          graphics
        )
      }
    });
  }

  Ok(())
}