extern crate core_lib;
extern crate core_ui;
extern crate core_piston;
extern crate specs;

mod win_gui;

use specs::prelude::*;
use core_ui::{resources::*, setup_core_ui};
use core_piston::new_piston_window;

fn main() {
  let mut world = World::new();
  let mut builder = DispatcherBuilder::new();

  // Setup the core ui
  setup_core_ui(&mut builder);

  // Add the window system and build the dispatcher
  builder.add(win_gui::GuiSystem, "gui_system", &[]);
  builder.add_thread_local(new_piston_window());

  // Build the dispatcher
  let mut dispatcher = builder.build();
  dispatcher.setup(&mut world);
  while !world.fetch::<ExitState>().is_exiting() {
    dispatcher.dispatch(&mut world);
  }
}