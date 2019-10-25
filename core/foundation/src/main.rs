#[macro_use] extern crate geopad_core_lib;
extern crate piston_window;
extern crate specs;
extern crate itertools;

use piston_window::*;
use specs::prelude::*;
use geopad_core_lib::setup_core_lib;

mod events;
mod resources;
mod systems;
mod utilities;

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
  builder.add(interactions::viewport::ViewportDragTool::default(), "viewport_drag_tool", &[]);
  builder.add(interactions::viewport::MoveViewportViaScroll::default(), "move_viewport_via_scroll", &[]);
  builder.add(interactions::history::UndoRedoViaKeyboard::default(), "undo_redo_via_keyboard", &[]);
  builder.add(interactions::geometry::point::SnapPointViaMouse::default(), "snap_point_via_mouse", &[]);
  builder.add(interactions::marker::SeldeViaMouse::default(), "selde_via_mouse", &[]);
  builder.add(interactions::marker::SeldeAllViaKeyboard::default(), "selde_all_via_keyboard", &[]);

  // Geometry interactions (not depend on snap point)
  builder.add(interactions::geometry::point::MovePointViaDrag::default(), "move_point_via_drag", &[]);
  builder.add(interactions::geometry::point::CreateMidpointViaKeyboard::default(), "create_midpoint_via_keyboard", &[]);
  builder.add(interactions::geometry::line::CreateParallelViaKeyboard::default(), "create_parallel_via_keyboard", &[]);
  builder.add(interactions::geometry::line::CreatePerpendicularViaKeyboard::default(), "create_perpendicular_via_keyboard", &[]);
  builder.add(interactions::geometry::RemoveSelectedViaKeyboard::default(), "remove_selected_via_keyboard", &[]);

  // Geometry creation (will depend on snap point)
  builder.add(interactions::geometry::point::CreatePointViaMouse::default(), "create_point_via_mouse", &["snap_point_via_mouse"]);
  builder.add(interactions::geometry::point::EmitActivePointEvent::default(), "emit_active_point_event", &[]);
  builder.add(interactions::geometry::point::ClickOnExistingPoint::default(), "click_on_existing_point", &["snap_point_via_mouse"]);
  builder.add(interactions::geometry::line::CreateLineViaMouse::default(), "create_line_via_mouse", &["emit_active_point_event", "click_on_existing_point"]);
  builder.add(interactions::geometry::circle::CreateCircleViaMouse::default(), "create_circle_via_mouse", &["emit_active_point_event", "click_on_existing_point"]);

  // State managers
  builder.add(state_managers::ExitStateManager::default(), "exit_state_manager", &["exit_via_keyboard"]);
  builder.add(state_managers::ToolStateManager::default(), "tool_state_manager", &["change_tool_via_keyboard", "change_line_tool_via_keyboard"]);

  // Setup the core library
  setup_core_lib(&mut builder);

  // Renderers
  builder.add(renderers::SnapPointRenderer::default(), "snap_point_renderer", &[]);
  builder.add(renderers::SnapLineRenderer::default(), "snap_line_renderer", &[]);
  builder.add(renderers::SnapCircleRenderer::default(), "snap_circle_renderer", &[]);
  builder.add(renderers::SelectRectangleRenderer::default(), "select_rectangle_renderer", &[]);

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