#[macro_use] extern crate geopad_core_lib;
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

  // Interaction related systems
  builder.add(interactions::exit::ExitViaKeyboard::default(), "exit_via_keyboard", &[]);
  builder.add(interactions::tool::ChangeToolViaKeyboard::default(), "change_tool_via_keyboard", &[]);
  builder.add(interactions::tool::ChangeLineToolViaKeyboard::default(), "change_line_tool_via_keyboard", &[]);
  builder.add(interactions::geometry::point::SnapPointViaMouse::default(), "snap_point_via_mouse", &[]);

  // Geometry creation will depend on snap point
  builder.add(interactions::geometry::point::CreatePointViaMouse::default(), "create_point_via_mouse", &["snap_point_via_mouse"]);

  // State managers
  builder.add(state_managers::ExitStateManager::default(), "exit_state_manager", &["exit_via_keyboard"]);
  builder.add(state_managers::ToolStateManager::default(), "tool_state_manager", &["change_tool_via_keyboard", "change_line_tool_via_keyboard"]);

  // Setup the core library
  setup_core_lib(&mut builder);

  // Renderers
  builder.add(renderers::SnapPointRenderer::default(), "snap_point_renderer", &[]);

  // Lastly, add the window system and build the dispatcher
  builder.add_thread_local(window_system);
  let mut dispatcher = builder.build();

  // Setup the world
  dispatcher.setup(&mut world);

  // Enter main loop
  while !world.fetch::<resources::ExitState>().is_exiting() {
    dispatcher.dispatch(&mut world);
  }
}