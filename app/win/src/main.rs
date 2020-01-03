#![windows_subsystem = "windows"]

extern crate core_lib;
extern crate core_ui;
extern crate geopad_foundation;
extern crate shrev;
extern crate specs;
#[cfg(target_os = "windows")]
#[macro_use]
extern crate lazy_static;
#[cfg(target_os = "windows")]
#[macro_use]
extern crate native_windows_gui as nwg;

extern crate open;
extern crate user32;
extern crate winapi;

#[cfg(target_os = "windows")]
mod win_gui;

#[cfg(target_os = "windows")]
fn main() {
  use core_ui::{resources::*, setup_core_ui};
  use geopad_foundation::new_piston_window;
  use specs::prelude::*;

  let mut world = World::new();
  let mut builder = DispatcherBuilder::new();

  // Setup the core ui
  setup_core_ui(&mut builder);

  // Add the window system and build the dispatcher
  builder.add_thread_local(new_piston_window());
  builder.add_thread_local(win_gui::GuiSystem::default());

  // Build the dispatcher
  let mut dispatcher = builder.build();
  dispatcher.setup(&mut world);
  while !world.fetch::<ExitState>().is_exiting() {
    dispatcher.dispatch(&mut world);
  }
}

#[cfg(not(target_os = "windows"))]
fn main() {
  println!("Not implemented");
}
