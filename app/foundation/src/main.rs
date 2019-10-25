#[macro_use] extern crate core_lib;
extern crate core_ui;
extern crate specs;
extern crate piston_window;

mod window_system;
mod utilities;

use specs::prelude::*;
use piston_window::*;
use core_ui::{resources::*, setup_core_ui};

fn main() {
  let mut world = World::new();

  // Initialize the window
  let window : PistonWindow = WindowSettings::new("Geometry Sketchpad", core_lib::resources::WINDOW_SIZE).build().unwrap();
  let window_system = window_system::WindowSystem { window };

  // Start the builder
  let mut builder = DispatcherBuilder::new();

  // Setup the core ui
  setup_core_ui(&mut builder);

  // Add the window system and build the dispatcher
  builder.add_thread_local(window_system);

  // Build the dispatcher
  let mut dispatcher = builder.build();

  // Setup the world
  dispatcher.setup(&mut world);

  // Enter main loop
  while !world.fetch::<ExitState>().is_exiting() {
    dispatcher.dispatch(&mut world);
  }
}