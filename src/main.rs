#![feature(type_alias_enum_variants)]
#![feature(duration_float)]

extern crate piston_window;
extern crate specs;
extern crate shrev;

#[macro_use] mod util;
mod components;
mod resources;
mod systems;

use piston_window::{PistonWindow, WindowSettings};
use specs::prelude::*;
use resources::*;
use systems::*;

fn main() {

  // Create a world
  let mut world = World::new();

  // Create a window
  let window : PistonWindow = WindowSettings::new("Geometry Sketchpad - Untitled.gsp", WINDOW_SIZE).build().unwrap();
  let window_system = WindowSystem { window };

  // Create dispatcher
  let mut dispatcher = DispatcherBuilder::new()

    // Interactions
    .with(interactions::ExitViaKeyboard, "exit_via_keyboard", &[])
    .with(interactions::ChangeToolViaKeyboard, "change_tool_via_keyboard", &[])
    .with(interactions::MoveViewportViaScroll, "move_viewport_via_scroll", &[])
    .with(interactions::SeldeAllViaKeyboard, "selde_all_via_keyboard", &[])
    .with(interactions::RemoveSelectedViaDelete, "remove_selected_via_delete", &[])
    .with(interactions::MouseEventEmitter::default(), "mouse_event_emitter", &[])

    // State Managers
    .with(state_managers::ExitStateManager::default(), "exit_state_manager", &["exit_via_keyboard"])
    .with(state_managers::ToolStateManager::default(), "tool_state_manager", &["change_tool_via_keyboard"])
    .with(state_managers::ViewportStateManager::default(), "viewport_state_manager", &["move_viewport_via_scroll"])

    // Data structures
    .with(cache_managers::DependencyGraphCache::default(), "dependency_graph_cache", &[])
    .with(cache_managers::SpatialHashCache::default(), "spatial_hash_cache", &["viewport_state_manager"])

    // Geometry action handlers
    .with(geometry_actions::MouseSelectSystem::default(), "mouse_select_system", &["mouse_event_emitter", "tool_state_manager"])
    .with(geometry_actions::SeldeAllHandler::default(), "selde_all_handler", &["selde_all_via_keyboard"])
    .with(geometry_actions::RemoveSelectedHandler::default(), "remove_selected_handler", &["remove_selected_via_delete", "dependency_graph_cache"])

    // Create geometry systems
    .with(geometry_systems::SeldeHandler::default(), "selde_handler", &["selde_all_handler"])
    .with(geometry_systems::RemoveHandler::default(), "geometry_remove_handler", &["remove_selected_handler"])

    .with(SnapPointSystem, "snap_point_system", &["spatial_hash_cache", "tool_state_manager"])
    .with(SnapPointRenderer::default(), "snap_point_renderer", &["snap_point_system"])
    .with(CreatePointSystem, "create_point_system", &["snap_point_system"])
    .with(CreateLineAbortSystem, "create_line_abort_system", &[])
    .with(CreateLineSystem::default(), "create_line_system", &["create_point_system"])
    .with(CreateLineRenderer::default(), "create_line_renderer", &["create_line_system"])

    // Solver & final rendering
    .with(geometry_systems::SolverSystem::default(), "solver_system", &["create_point_system", "create_line_system"])
    .with_thread_local(window_system)
    .build();

  // Setup resources
  dispatcher.setup(&mut world);

  // Enter game main loop
  while world.fetch::<state_managers::ExitState>().is_running() {
    dispatcher.dispatch(&mut world);
  }
}