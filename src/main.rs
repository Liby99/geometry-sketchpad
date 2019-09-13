extern crate piston_window;

mod geometry;
mod storage;
mod math;
mod context;

use math::Vec2;
use geometry::{
  point::{ Point, PointConstruct },
  line::{ Line, LineConstruct },
};

// use piston_window::*;

fn main() -> Result<(), context::SolveError> {

  let mut context = context::Context::new();

  let pa = context.points.add(PointConstruct::Free { pos: Vec2::new(-2., 0.) });
  let pb = context.points.add(PointConstruct::Free { pos: Vec2::new(-1., 1.) });
  let pc = context.points.add(PointConstruct::Free { pos: Vec2::new(1., 1.) });
  let pd = context.points.add(PointConstruct::Free { pos: Vec2::new(2., 0.) });

  let l1 = context.lines.add(LineConstruct::TwoPoint { p1: pa, p2: pb });
  let l2 = context.lines.add(LineConstruct::TwoPoint { p1: pd, p2: pc });
  let x_axis = context.lines.add(LineConstruct::TwoPoint { p1: pa, p2: pd });

  let pe = context.points.add(PointConstruct::LineLineIntersect { l1, l2 });

  let l3 = context.lines.add(LineConstruct::Parallel { l: x_axis, p: pe });

  let solution = context.solve()?;

  println!("{:?}", solution);

  Ok(())

  // let mut window: PistonWindow =
  //     WindowSettings::new("Hello Piston!", [640, 480])
  //     .exit_on_esc(true).build().unwrap();
  // while let Some(event) = window.next() {
  //     window.draw_2d(&event, |context, graphics, _device| {
  //         clear([1.0; 4], graphics);
  //         rectangle([1.0, 0.0, 0.0, 1.0], // red
  //                   [0.0, 0.0, 100.0, 100.0],
  //                   context.transform,
  //                   graphics);
  //     });
  // }
}