extern crate geopad_core_lib;
extern crate piston_window;
extern crate specs;

use piston_window::*;
use specs::prelude::*;
use geopad_core_lib::setup_core_lib;

mod events;
mod resources;
mod systems;

use systems::*;

fn main() {
  let mut world = World::new();

  // Initialize the window
  let window : PistonWindow = WindowSettings::new("Geometry Sketchpad", geopad_core_lib::resources::WINDOW_SIZE).build().unwrap();
  let window_system = window_system::WindowSystem { window };

  // Start the builder
  let mut builder = DispatcherBuilder::new();
  builder.add(state_managers::ExitStateManager::default(), "exit_state_manager", &[]);

  // Setup the core library
  setup_core_lib(&mut builder);

  // Add the window system and build the dispatcher
  builder.add_thread_local(window_system);
  let mut dispatcher = builder.build();

  // Setup the world
  dispatcher.setup(&mut world);

  // Enter main loop
  while !world.fetch::<resources::ExitState>().is_exiting() {
    dispatcher.dispatch(&mut world);
  }
}