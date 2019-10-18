use std::mem::drop;
// use std::collections::HashSet;
use specs::prelude::*;
use crate::{
  resources::{
    ToolState, Tool, // DependencyGraph,
    geometry::{LastActivePointReader, LastActivePointChannel, CreateLineData},
    events::{GeometryAction, GeometryActionChannel},
  },
  components::{/*SymbolicPoint, */SymbolicCircle},
};

pub struct CreateCenRadCircleViaMouse {
  last_active_point_reader: Option<LastActivePointReader>,
}

impl Default for CreateCenRadCircleViaMouse {
  fn default() -> Self {
    Self { last_active_point_reader: None }
  }
}

impl<'a> System<'a> for CreateCenRadCircleViaMouse {
  type SystemData = (
    Read<'a, ToolState>,
    // Read<'a, DependencyGraph>,
    Write<'a, CreateLineData>,
    Write<'a, LastActivePointChannel>,
    Write<'a, GeometryActionChannel>,
    // ReadStorage<'a, SymbolicPoint>,
    // ReadStorage<'a, SymbolicCircle>,
  );

  fn run(&mut self, (
    tool_state,
    // dependency_graph,
    mut create_line_data,
    mut last_active_point_channel,
    mut geometry_action_channel,
    // sym_points,
    // sym_circles,
  ): Self::SystemData) {

    // First deal with tooling states
    if let Some(reader_id) = &mut self.last_active_point_reader {
      match tool_state.get() {
        Tool::Circle => (),
        _ => {
          drop(reader_id);
          self.last_active_point_reader = None;
          create_line_data.maybe_first_point = None;
        }
      }
    } else {
      match tool_state.get() {
        Tool::Circle => {
          self.last_active_point_reader = Some(last_active_point_channel.register_reader());
        },
        _ => ()
      }
    }

    // Note that if the reader id is None, then we are not using create line tool
    // If this goes into the branch, it is guarenteed that we are using line tool, and we will be listening to the event from then on
    if let Some(reader_id) = &mut self.last_active_point_reader {
      for event in last_active_point_channel.read(reader_id) {
        let curr_point_entity = event.get();
        if let Some(first_point_entity) = create_line_data.maybe_first_point {

          // Need to check first point is not second point
          if first_point_entity != curr_point_entity { // && !on_same_line(first_point_entity, curr_point_entity, &dependency_graph, &sym_points, &sym_lines) {
            let sym_circle = SymbolicCircle::CenterRadius(first_point_entity, curr_point_entity);

            // Push event to created lines
            geometry_action_channel.single_write(GeometryAction::InsertCircle(sym_circle));

            // Reset the maybe first point
            create_line_data.maybe_first_point = None;
          }
        } else {

          // If there's no first point, then set the current point to the first point
          create_line_data.maybe_first_point = Some(curr_point_entity);
        }

        // We only deal with one event
        break;
      }
    }
  }
}