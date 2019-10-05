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
    .with(ExitSystem, "exit_system", &[])
    .with(ViewportSystem, "viewport_system", &[])
    .with(ChangeToolSystem, "change_tool_system", &[])

    // Data structures
    .with(DependencyGraphCache::default(), "dependency_graph_cache", &[])
    .with(SpatialHashCache::default(), "spatial_hash_cache", &["viewport_system"])

    // Create geometry systems
    .with(SelectSystem, "select_system", &["spatial_hash_cache"])
    .with(SnapPointSystem, "snap_point_system", &["spatial_hash_cache", "change_tool_system"])
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
  while world.fetch::<FinishState>().not_finished() {
    dispatcher.dispatch(&mut world);
  }
}