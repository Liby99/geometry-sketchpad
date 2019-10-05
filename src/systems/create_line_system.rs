use std::mem::drop;
use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::{
  util::Color,
  resources::{ToolState, LastActivePoint, SketchEvent, Geometry, CreateLineData},
  components::{SymbolicLine, LineStyle, Selected},
};

pub struct CreateLineSystem {
  last_active_point_event_reader_id: Option<ReaderId<LastActivePoint>>,
}

impl Default for CreateLineSystem {
  fn default() -> Self {
    Self { last_active_point_event_reader_id: None }
  }
}

impl<'a> System<'a> for CreateLineSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolState>,
    Write<'a, CreateLineData>,
    Write<'a, EventChannel<LastActivePoint>>,
    Write<'a, EventChannel<SketchEvent>>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    tool_state,
    mut create_line_data,
    mut last_active_point_event,
    mut sketch_events,
    mut sym_lines,
    mut styles,
    mut selected,
  ): Self::SystemData) {

    // First deal with tooling states
    if let Some(reader_id) = &mut self.last_active_point_event_reader_id {
      match *tool_state {
        ToolState::Line => (),
        _ => {
          drop(reader_id);
          self.last_active_point_event_reader_id = None;
          create_line_data.maybe_first_point = None;
        }
      }
    } else {
      match *tool_state {
        ToolState::Line => {
          self.last_active_point_event_reader_id = Some(last_active_point_event.register_reader());
        },
        _ => ()
      }
    }

    // Note that if the reader id is None, then we are not using create line tool
    // If this goes into the branch, it is guarenteed that we are using line tool, and we will be listening to the event from then on
    if let Some(reader_id) = &mut self.last_active_point_event_reader_id {
      for event in last_active_point_event.read(reader_id) {
        let curr_point_entity = event.get();
        if let Some(first_point_entity) = create_line_data.maybe_first_point {

          // Create a new point from `first_point_entity` to `curr_entity`
          let entity = entities.create();
          if let Err(err) = sym_lines.insert(entity, SymbolicLine::TwoPoints(first_point_entity, curr_point_entity)) { panic!(err) }
          if let Err(err) = styles.insert(entity, LineStyle { color: Color::blue(), width: 2. }) { panic!(err) }
          if let Err(err) = selected.insert(entity, Selected) { panic!(err) }

          // Push event to created lines
          sketch_events.single_write(SketchEvent::Inserted(entity, Geometry::Line));

          // Reset the maybe first point
          create_line_data.maybe_first_point = None;

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