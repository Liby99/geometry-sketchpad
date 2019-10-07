extern crate piston_window;
extern crate specs;
extern crate shrev;

#[macro_use] mod utilities;
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
    .with(interactions::select::SeldeAllViaKeyboard, "selde_all_via_keyboard", &[])
    .with(interactions::remove::RemoveSelectedViaDelete, "remove_selected_via_delete", &[])
    .with(interactions::create::line::AbortCreateLineViaKeyboard, "abort_create_line_via_keyboard", &[])
    .with(interactions::history::UndoViaKeyboard, "undo_via_keyboard", &[])
    .with(interactions::history::RedoViaKeyboard, "redo_via_keyboard", &[])

    // We put tooling handler here first
    .with(state_managers::ToolStateManager::default(), "tool_state_manager", &["change_tool_via_keyboard"])

    // Interations based on tool
    .with(interactions::MoveViewportViaDrag::default(), "move_viewport_via_drag", &["tool_state_manager"])
    .with(interactions::select::SeldeViaMouse::default(), "selde_via_mouse", &["tool_state_manager"])
    .with(interactions::update::MovePointViaDrag::default(), "move_point_via_drag", &["tool_state_manager"])
    .with(interactions::create::line::CreateParallelLineViaKeyboard, "create_parallel_line_via_keyboard", &[])
    .with(interactions::create::line::CreatePerpLineViaKeyboard, "create_perp_line_via_keyboard", &[])
    .with(interactions::create::point::CreateMidpointViaKeyboard, "create_midpoint_via_keyboard", &[])

    // Other state Managers
    .with(state_managers::ExitStateManager::default(), "exit_state_manager", &["exit_via_keyboard"])
    .with(state_managers::ViewportStateManager::default(), "viewport_state_manager", &["move_viewport_via_scroll", "move_viewport_via_drag"])

    // Data structures
    .with(cache_managers::DependencyGraphCache::default(), "dependency_graph_cache", &[])
    .with(cache_managers::SpatialHashCache::default(), "spatial_hash_cache", &["viewport_state_manager"])

    // Geometry action handlers
    .with(geometry_actions::SeldeAllHandler::default(), "selde_all_handler", &["selde_all_via_keyboard", "selde_via_mouse"])
    .with(geometry_actions::RemoveSelectedHandler::default(), "remove_selected_handler", &["remove_selected_via_delete", "dependency_graph_cache"])

    // Geometry helpers
    .with(interactions::create::point::SnapPointSystem, "snap_point_system", &["spatial_hash_cache", "tool_state_manager", "viewport_state_manager"])

    // Create geometry systems
    .with(geometry_systems::SeldeHandler::default(), "selde_handler", &["selde_all_handler"])
    .with(geometry_systems::MovePointHandler::default(), "move_point_handler", &["move_point_via_drag"])

    // Create point
    .with(interactions::create::point::CreatePointViaMouse::default(), "create_point_via_mouse", &["snap_point_system"])

    // Create geometry interactions
    .with(interactions::create::line::CreateTwoPointLineViaMouse::default(), "create_two_point_line_via_mouse", &["create_point_via_mouse"])
    .with(geometry_actions::DrawParallelOnSelected::default(), "draw_parallel_on_selected", &["create_parallel_line_via_keyboard", "selde_handler"])
    .with(geometry_actions::DrawPerpOnSelected::default(), "draw_perp_on_selected", &["create_perp_line_via_keyboard", "selde_handler"])
    .with(geometry_actions::DrawMidpointOnSelected::default(), "draw_midpoint_on_selected", &["create_midpoint_via_keyboard", "selde_handler"])

    // History manipulating world states
    .with(cache_managers::SketchHistoryActionHandler::default(), "sketch_history_action_handler", &["undo_via_keyboard", "redo_via_keyboard", "spatial_hash_cache"]) // It needs to run after spatial hash cache

    // Insert systems
    .with(geometry_systems::InsertNewPointSystem::default(), "insert_point_system", &["create_point_via_mouse"])
    .with(geometry_systems::InsertNewLineSystem::default(), "insert_line_system", &["create_parallel_line_via_keyboard", "create_two_point_line_via_mouse"])
    .with(geometry_systems::InsertHistoryGeometry::default(), "insert_history_geometry", &["sketch_history_action_handler"])

    // Remove systems
    .with(geometry_systems::RemoveHandler::default(), "remove_handler", &["remove_selected_handler", "sketch_history_action_handler"])

    // History caching
    .with(cache_managers::SketchHistoryCache::default(), "sketch_history_cache", &["insert_point_system", "insert_line_system", "remove_handler"])

    // Renderers
    .with(geometry_renderers::SnapPointRenderer::default(), "snap_point_renderer", &["snap_point_system"])
    .with(geometry_renderers::CreateLineRenderer::default(), "create_line_renderer", &["create_two_point_line_via_mouse"])
    .with(geometry_renderers::SelectRectangleRenderer::default(), "select_rectangle_renderer", &["selde_via_mouse"])

    // Solver & final rendering
    .with(geometry_systems::SolverSystem::default(), "solver_system", &["insert_point_system", "insert_line_system", "insert_history_geometry", "remove_handler"])
    .with_thread_local(window_system)
    .build();

  // Setup resources
  dispatcher.setup(&mut world);

  // Enter game main loop
  while world.fetch::<state_managers::ExitState>().is_running() {
    dispatcher.dispatch(&mut world);
  }
}