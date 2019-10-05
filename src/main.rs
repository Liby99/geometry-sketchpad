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
    .with(DragEventEmitter::default(), "drag_event_emitter", &[])

    // Interactions
    .with(interactions::ExitViaKeyboard, "exit_via_keyboard", &[])
    .with(interactions::ChangeToolViaKeyboard, "change_tool_via_keyboard", &[])
    .with(interactions::MoveViewportViaScroll, "move_viewport_via_scroll", &[])

    // State Managers
    .with(state_managers::ExitStateManager::default(), "exit_state_manager", &["exit_via_keyboard"])
    .with(state_managers::ToolStateManager::default(), "tool_state_manager", &["change_tool_via_keyboard"])
    .with(state_managers::ViewportStateManager::default(), "viewport_state_manager", &["move_viewport_via_scroll"])

    // Data structures
    .with(DependencyGraphCache::default(), "dependency_graph_cache", &[])
    .with(SpatialHashCache::default(), "spatial_hash_cache", &["viewport_state_manager"])

    // Create geometry systems
    .with(RemoveGeomSystem, "remove_geom_system", &["dependency_graph_cache"])
    .with(SelectSystem, "select_system", &["spatial_hash_cache"])
    .with(SnapPointSystem, "snap_point_system", &["spatial_hash_cache", "tool_state_manager"])
    .with(SnapPointRenderer::default(), "snap_point_renderer", &["snap_point_system"])
    .with(CreatePointSystem, "create_point_system", &["snap_point_system"])
    .with(CreateLineAbortSystem, "create_line_abort_system", &[])
    .with(CreateLineSystem::default(), "create_line_system", &["create_point_system"])
    .with(CreateLineRenderer::default(), "create_line_renderer", &["create_line_system"])

    // Solver & final rendering
    .with(SolverSystem::default(), "solver_system", &["create_point_system", "create_line_system"])
    .with_thread_local(window_system)
    .build();

  // Setup resources
  dispatcher.setup(&mut world);

  // Enter game main loop
  while world.fetch::<state_managers::ExitState>().is_running() {
    dispatcher.dispatch(&mut world);
  }
}