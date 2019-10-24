extern crate specs;

use specs::prelude::*;

#[macro_use]
pub mod math;
pub mod utilities;
pub mod components;
pub mod resources;
pub mod events;
pub mod systems;

use systems::*;

pub fn setup_core_lib<'a, 'b>(builder: &mut DispatcherBuilder<'a, 'b>) {
  builder.add_barrier();
  builder.add(event_handlers::ViewportEventHandler::default(), "viewport_event_handler", &[]);
  builder.add(event_handlers::HistoryEventHandler::default(), "history_event_handler", &[]);
  builder.add(command_handlers::RemoveHandler::default(), "remove_handler", &["history_event_handler"]);
  builder.add(command_handlers::InsertPointHandler::default(), "insert_point_handler", &["history_event_handler"]);
  builder.add(command_handlers::InsertLineHandler::default(), "insert_line_handler", &["history_event_handler"]);
  builder.add(command_handlers::InsertCircleHandler::default(), "insert_circle_handler", &["history_event_handler"]);
  builder.add(command_handlers::HideHandler::default(), "hide_handler", &["history_event_handler"]);
  builder.add(command_handlers::SelectHandler::default(), "select_handler", &["history_event_handler"]);
  builder.add(data_managers::HistoryManager::default(), "history_manager", &["remove_handler", "insert_point_handler", "insert_line_handler", "insert_circle_handler", "hide_handler"]);
  builder.add(data_managers::DependencyGraphManager::default(), "dependency_graph_manager", &["remove_handler", "insert_point_handler", "insert_line_handler", "insert_circle_handler"]);
  builder.add(solvers::VirtualShapeSolver::default(), "virtual_shape_solver", &["dependency_graph_manager"]);
  builder.add(solvers::ScreenShapeSolver::default(), "screen_shape_solver", &["virtual_shape_solver"]);
  builder.add(data_managers::SpatialEntityMapManager::default(), "spatial_entity_map_manager", &["screen_shape_solver"]);
  builder.add_barrier();
}