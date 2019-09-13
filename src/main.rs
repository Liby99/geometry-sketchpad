extern crate piston_window;
extern crate specs;

mod systems;
mod states;
mod components;
mod math;
mod util;

use piston_window::*;
use specs::prelude::*;
use states::FinishState;
use systems::{RenderSystem, SolverSystem};
use components::{point, line};
use util::Color;
use math::Vector2;

fn main() {

  // Create a world
  let mut world = World::new();
  world.insert(FinishState(false));
  world.register::<point::Point>();
  world.register::<point::SymbolicPoint>();
  world.register::<point::PointStyle>();
  world.register::<line::Line>();
  world.register::<line::SymbolicLine>();
  world.register::<line::LineStyle>();

  // ============ TEMP START ============
  let p1 = world.create_entity()
    .with(point::PointStyle { color: Color::red(), radius: 5. })
    .with(point::SymbolicPoint::Free(Vector2 { x: 0., y: 0. }))
    .build();

  let p2 = world.create_entity()
    .with(point::PointStyle { color: Color::red(), radius: 5. })
    .with(point::SymbolicPoint::Free(Vector2 { x: 30., y: 10. }))
    .build();

  let p3 = world.create_entity()
    .with(point::PointStyle { color: Color::red(), radius: 5. })
    .with(point::SymbolicPoint::Free(Vector2 { x: -20., y: -20. }))
    .build();

  let l1 = world.create_entity()
    .with(line::LineStyle { color: Color::blue(), width: 2. })
    .with(line::SymbolicLine::TwoPoints(p1, p3))
    .build();

  let p4 = world.create_entity()
    .with(point::PointStyle { color: Color::green(), radius: 5. })
    .with(point::SymbolicPoint::OnLine(l1, -40.))
    .build();

  let _l2 = world.create_entity()
    .with(line::LineStyle { color: Color::black(), width: 2. })
    .with(line::SymbolicLine::TwoPoints(p2, p4))
    .build();
  // ============ TEMP END ============

  // Create a window
  let window : PistonWindow = WindowSettings::new("Geometry Sketchpad - Untitled.gsp", [960, 720]).build().unwrap();
  let render_system = RenderSystem { window };

  // Create dispatcher
  let mut dispatcher = DispatcherBuilder::new()
    .with(SolverSystem, "solver", &[])
    .with_thread_local(render_system)
    .build();

  // Enter game main loop
  while world.fetch::<FinishState>().not_finished() {
    dispatcher.dispatch(&mut world);
  }
}