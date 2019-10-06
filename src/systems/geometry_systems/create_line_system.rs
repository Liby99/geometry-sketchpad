use std::mem::drop;
use std::collections::HashSet;
use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::{
  utilities::Color,
  resources::{
    ToolState, Tool, DependencyGraph,
    geometry::{LastActivePoint, CreateLineData},
    events::{SketchEvent, Geometry, SketchEventChannel},
  },
  components::{SymbolicPoint, SymbolicLine, LineStyle, Selected},
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
    Read<'a, DependencyGraph>,
    Write<'a, CreateLineData>,
    Write<'a, EventChannel<LastActivePoint>>,
    Write<'a, SketchEventChannel>,
    ReadStorage<'a, SymbolicPoint>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    tool_state,
    dependency_graph,
    mut create_line_data,
    mut last_active_point_event,
    mut sketch_events,
    sym_points,
    mut sym_lines,
    mut styles,
    mut selected,
  ): Self::SystemData) {

    // First deal with tooling states
    if let Some(reader_id) = &mut self.last_active_point_event_reader_id {
      match tool_state.get() {
        Tool::Line => (),
        _ => {
          drop(reader_id);
          self.last_active_point_event_reader_id = None;
          create_line_data.maybe_first_point = None;
        }
      }
    } else {
      match tool_state.get() {
        Tool::Line => {
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

          // Need to check first point is not second point
          if first_point_entity != curr_point_entity &&
            !on_same_line(first_point_entity, curr_point_entity, &dependency_graph, &sym_points, &sym_lines) {

            let sym_line = SymbolicLine::TwoPoints(first_point_entity, curr_point_entity);
            let line_style = LineStyle { color: Color::blue(), width: 2. };

            // Create a new point from `first_point_entity` to `curr_entity`
            let entity = entities.create();
            if let Err(err) = sym_lines.insert(entity, sym_line) { panic!(err) }
            if let Err(err) = styles.insert(entity, line_style) { panic!(err) }
            if let Err(err) = selected.insert(entity, Selected) { panic!(err) }

            // Push event to created lines
            sketch_events.single_write(SketchEvent::Insert(entity, Geometry::Line(sym_line, line_style)));

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

fn on_same_line<'a>(
  p1: Entity,
  p2: Entity,
  dependency_graph: &DependencyGraph,
  sym_points: &ReadStorage<'a, SymbolicPoint>,
  sym_lines: &WriteStorage<'a, SymbolicLine>,
) -> bool {
  if let Some(sp1) = sym_points.get(p1) {
    if let Some(sp2) = sym_points.get(p2) {
      let direct_check_on_same_line = sp1.is_on_same_line_with(sp2);
      if direct_check_on_same_line {
        return true;
      } else {
        if let Some(p1_children) = dependency_graph.get_direct_dependents(&p1) {
          if let Some(p2_children) = dependency_graph.get_direct_dependents(&p2) {
            for itsct in p1_children.intersection(p2_children) {
              if sym_lines.get(*itsct).is_some() {
                return true;
              }
            }
            if check_parent_line_contained_by(sp1, p2_children) {
              return true;
            }
            if check_parent_line_contained_by(sp2, p1_children) {
              return true;
            }
            return false;
          } else {
            return check_parent_line_contained_by(sp2, p1_children);
          }
        } else {
          if let Some(p2_children) = dependency_graph.get_direct_dependents(&p2) {
            return check_parent_line_contained_by(sp1, p2_children);
          } else {
            return false;
          }
        }
      }
    }
  }
  panic!("[create_line_system] Point entities does not have symbolic point");
}

fn check_parent_line_contained_by(sp: &SymbolicPoint, set: &HashSet<Entity>) -> bool {
  match sp {
    SymbolicPoint::Free(_) => false,
    SymbolicPoint::OnLine(line_ent, _) => set.contains(&line_ent),
    SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => set.contains(&l1_ent) || set.contains(&l2_ent),
  }
}