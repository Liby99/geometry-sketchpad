extern crate piston_window;
extern crate specs;

mod systems;
mod states;
mod components;

use piston_window::*;
use specs::prelude::*;
use states::FinishState;
use systems::RenderSystem;

fn main() {

  // Create a world
  let mut world = World::new();
  world.insert(FinishState(false));

  // Create a window
  let window : PistonWindow = WindowSettings::new("Geometry Sketchpad - Untitled.gsp", [960, 720]).build().unwrap();
  let render_system = RenderSystem { window };

  // Create dispatcher
  let mut dispatcher = DispatcherBuilder::new()
    .with_thread_local(render_system)
    .build();

  // Enter game main loop
  while world.fetch::<FinishState>().not_finished() {
    dispatcher.dispatch(&mut world);
  }
}