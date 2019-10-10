use std::mem::drop;
use std::collections::HashSet;
use specs::prelude::*;
use crate::{
  resources::{
    ToolState, Tool, LineTool, DependencyGraph,
    geometry::{LastActivePointReader, LastActivePointChannel, CreateLineData},
    events::{GeometryAction, GeometryActionChannel},
  },
  components::{SymbolicPoint, SymbolicLine},
};

pub struct CreateTwoPointLineViaMouse {
  last_active_point_reader: Option<LastActivePointReader>,
}

impl Default for CreateTwoPointLineViaMouse {
  fn default() -> Self {
    Self { last_active_point_reader: None }
  }
}

impl<'a> System<'a> for CreateTwoPointLineViaMouse {
  type SystemData = (
    Read<'a, ToolState>,
    Read<'a, DependencyGraph>,
    Write<'a, CreateLineData>,
    Write<'a, LastActivePointChannel>,
    Write<'a, GeometryActionChannel>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
  );

  fn run(&mut self, (
    tool_state,
    dependency_graph,
    mut create_line_data,
    mut last_active_point_channel,
    mut geometry_action_channel,
    sym_points,
    sym_lines,
  ): Self::SystemData) {

    // First deal with tooling states
    if let Some(reader_id) = &mut self.last_active_point_reader {
      match tool_state.get() {
        Tool::Line(_) => (),
        _ => {
          drop(reader_id);
          self.last_active_point_reader = None;
          create_line_data.maybe_first_point = None;
        }
      }
    } else {
      match tool_state.get() {
        Tool::Line(_) => {
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
          if first_point_entity != curr_point_entity && !on_same_line(first_point_entity, curr_point_entity, &dependency_graph, &sym_points, &sym_lines) {
            if let Tool::Line(line_tool) = tool_state.get() {
              let sym_line = match line_tool {
                LineTool::Line => SymbolicLine::TwoPoints(first_point_entity, curr_point_entity),
                LineTool::Ray => SymbolicLine::Ray(first_point_entity, curr_point_entity),
                LineTool::Segment => SymbolicLine::Segment(first_point_entity, curr_point_entity), // TODO
              };

              // Push event to created lines
              geometry_action_channel.single_write(GeometryAction::InsertLine(sym_line));

              // Reset the maybe first point
              create_line_data.maybe_first_point = None;
            }
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
  sym_lines: &ReadStorage<'a, SymbolicLine>,
) -> bool {
  if let Some(sp1) = sym_points.get(p1) {
    if let Some(sp2) = sym_points.get(p2) {
      let direct_check_on_same_line = easy_check_is_on_same_line(sp1, sp2);
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

fn easy_check_is_on_same_line(sp1: &SymbolicPoint, sp2: &SymbolicPoint) -> bool {
  match sp1 {
    SymbolicPoint::Free(_) | SymbolicPoint::MidPoint(_, _) => false,
    SymbolicPoint::OnLine(line_ent, _) => match sp2 {
      SymbolicPoint::Free(_) | SymbolicPoint::MidPoint(_, _) => false,
      SymbolicPoint::OnLine(l1_ent, _) => line_ent == l1_ent,
      SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => line_ent == l1_ent || line_ent == l2_ent,
      SymbolicPoint::OnCircle(_, _) => false,
      SymbolicPoint::CircleLineIntersect(_, l_ent, _) => line_ent == l_ent,
      SymbolicPoint::CircleCircleIntersect(_, _, _) => false,
    },
    SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => match sp2 {
      SymbolicPoint::Free(_) | SymbolicPoint::MidPoint(_, _) => false,
      SymbolicPoint::OnLine(line_ent, _) => l1_ent == line_ent || l2_ent == line_ent,
      SymbolicPoint::LineLineIntersect(l3_ent, l4_ent) => {
        l1_ent == l3_ent || l1_ent == l4_ent || l2_ent == l3_ent || l2_ent == l4_ent
      },
      SymbolicPoint::OnCircle(_, _) => false,
      SymbolicPoint::CircleLineIntersect(_, l_ent, _) => l1_ent == l_ent || l2_ent == l_ent,
      SymbolicPoint::CircleCircleIntersect(_, _, _) => false,
    },
    SymbolicPoint::OnCircle(_, _) => false,
    SymbolicPoint::CircleLineIntersect(_, line_ent, _) => match sp2 {
      SymbolicPoint::Free(_) | SymbolicPoint::MidPoint(_, _) => false,
      SymbolicPoint::OnLine(l_ent, _) => line_ent == l_ent,
      SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => line_ent == l1_ent || line_ent == l2_ent,
      SymbolicPoint::OnCircle(_, _) => false,
      SymbolicPoint::CircleLineIntersect(_, l_ent, _) => line_ent == l_ent,
      SymbolicPoint::CircleCircleIntersect(_, _, _) => false,
    },
    SymbolicPoint::CircleCircleIntersect(_, _, _) => false,
  }
}

fn check_parent_line_contained_by(sp: &SymbolicPoint, set: &HashSet<Entity>) -> bool {
  match sp {
    SymbolicPoint::Free(_) | SymbolicPoint::MidPoint(_, _) => false,
    SymbolicPoint::OnLine(line_ent, _) => set.contains(&line_ent),
    SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => set.contains(&l1_ent) || set.contains(&l2_ent),
    SymbolicPoint::OnCircle(_, _) => false,
    SymbolicPoint::CircleLineIntersect(_, l, _) => set.contains(&l),
    SymbolicPoint::CircleCircleIntersect(_, _, _) => false,
  }
}