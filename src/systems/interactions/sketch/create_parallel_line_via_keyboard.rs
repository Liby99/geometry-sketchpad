use specs::prelude::*;
use crate::{
  utilities::Key,
  resources::{
    InputState,
    DependencyGraph,
    events::{InsertEvent, InsertEventChannel},
  },
  components::{SymbolicLine, SymbolicPoint, Selected},
};

pub struct CreateParallelLineViaKeyboard;

impl<'a> System<'a> for CreateParallelLineViaKeyboard {
  type SystemData = (
    Entities<'a>,
    Read<'a, InputState>,
    Read<'a, DependencyGraph>,
    Write<'a, InsertEventChannel>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    input_state,
    dependency_graph,
    mut insert_event_channel,
    sym_points,
    sym_lines,
    selected,
  ): Self::SystemData) {
    if input_state.keyboard.just_activated(Key::Minus) && input_state.keyboard.is_shift_activated() && input_state.keyboard.is_command_activated() {
      let mut maybe_line_ent = None;
      for (entity, _, _) in (&entities, &sym_lines, &selected).join() {
        if maybe_line_ent.is_none() {
          maybe_line_ent = Some(entity);
        } else {
          return; // Early terminate since we don't accept more than one line being selected
        }
      }

      if let Some(line_ent) = maybe_line_ent {
        for (p_ent, sym_point, _) in (&entities, &sym_points, &selected).join() {
          if !is_on_line(&p_ent, &sym_point, &line_ent, &dependency_graph) {
            let sym_line = SymbolicLine::Parallel(line_ent, p_ent);
            insert_event_channel.single_write(InsertEvent::Line(sym_line));
          }
        }
      }
    }
  }
}

fn is_on_line(p_ent: &Entity, sym_point: &SymbolicPoint, line_ent: &Entity, dependency_graph: &DependencyGraph) -> bool {
  match sym_point {
    SymbolicPoint::Free(_) => (),
    SymbolicPoint::OnLine(l, _) => {
      if *l == *line_ent {
        return true;
      }
    },
    SymbolicPoint::LineLineIntersect(l1, l2) => {
      if *l1 == *line_ent || *l2 == *line_ent {
        return true;
      }
    }
  }
  if let Some(dependents) = dependency_graph.get_direct_dependents(p_ent) {
    return dependents.contains(line_ent);
  }
  return false;
}