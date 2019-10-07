use specs::prelude::*;
use crate::{
  utilities::{Key, Color},
  resources::{
    InputState,
    DependencyGraph,
    events::{SketchEvent, SketchEventChannel, Geometry},
  },
  components::{SymbolicLine, SymbolicPoint, Selected, LineStyle},
};

pub struct CreateParallelLineViaKeyboard;

impl<'a> System<'a> for CreateParallelLineViaKeyboard {
  type SystemData = (
    Entities<'a>,
    Read<'a, InputState>,
    Read<'a, DependencyGraph>,
    Write<'a, SketchEventChannel>,
    ReadStorage<'a, SymbolicPoint>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    input_state,
    dependency_graph,
    mut sketch_event_channel,
    sym_points,
    mut sym_lines,
    mut line_styles,
    mut selected,
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

        let mut to_insert = vec![]; // Because we cannot mutate borrow. Delay the insertion to selected

        for (p_ent, sym_point, _) in (&entities, &sym_points, &selected).join() {
          if !is_on_line(&p_ent, &sym_point, &line_ent, &dependency_graph) {

            let sym_line = SymbolicLine::Parallel(line_ent, p_ent);
            let line_style = LineStyle { color: Color::blue(), width: 2. };

            // Create a new point from `first_point_entity` to `curr_entity`
            let entity = entities.create();
            if let Err(err) = sym_lines.insert(entity, sym_line) { panic!(err) }
            if let Err(err) = line_styles.insert(entity, line_style) { panic!(err) }

            to_insert.push(entity);

            // Also write the event
            sketch_event_channel.single_write(SketchEvent::Insert(entity, Geometry::Line(sym_line)));
          }
        }

        for entity in to_insert {
          if let Err(err) = selected.insert(entity, Selected) { panic!(err) }
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