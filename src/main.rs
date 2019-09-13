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
use components::point::*;
use math::Vector2;

fn main() {

  // Create a world
  let mut world = World::new();
  world.insert(FinishState(false));
  world.register::<Point>();
  world.register::<SymbolicPoint>();
  world.register::<PointStyle>();

  // ============ TEMP START ============
  world.create_entity()
    .with(PointStyle { color: [1., 0., 0., 1.], radius: 5. })
    .with(SymbolicPoint::Free(Vector2 { x: 0., y: 0. }))
    .build();

  world.create_entity()
    .with(PointStyle { color: [1., 0., 0., 1.], radius: 5. })
    .with(SymbolicPoint::Free(Vector2 { x: 30., y: 10. }))
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