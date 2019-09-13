// extern crate piston_window;

// use std::collections::BTreeMap;
use piston_window::*;
use specs::prelude::*;
use geopad::{
  ui::{
    systems::RenderSystem,
    states::FinishState,
  },
  // util::{Id, Storage},
  // geometry::Context,
};

fn main() {

  // Create a world
  let mut world = World::new();
  world.insert(FinishState(false));

  // Create a window
  let window : PistonWindow = WindowSettings::new("GeoPad", [960, 720]).build().unwrap();
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