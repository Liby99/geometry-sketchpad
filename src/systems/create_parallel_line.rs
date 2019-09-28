use specs::prelude::*;
use piston_window::Key;
use crate::{
  util::Color,
  math::Vector2,
  resources::{InputState, DirtyState, DescendantMap},
  components::{Selected, Line, SymbolicLine, LineStyle, Point},
};

pub struct CreateParallelLine;

impl<'a> System<'a> for CreateParallelLine {
  type SystemData = (
    Entities<'a>,
    Read<'a, InputState>,
    Write<'a, DirtyState>,
    Write<'a, DescendantMap>, // This does not really need to have Write access
    ReadStorage<'a, Line>,
    ReadStorage<'a, Point>,
    WriteStorage<'a, Selected>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
  );

  fn run(&mut self, (
    entities,
    input,
    mut dirty_state,
    mut descendent_map,
    lines,
    points,
    mut selected,
    mut sym_lines,
    mut line_styles
  ): Self::SystemData) {
    if input.keyboard.just_activated_with_shift(Key::Minus) {
      let mut maybe_line : Option<(Entity, Vector2)> = None; // Entity, Direction
      for (line_ent, line, _) in (&*entities, &lines, &selected).join() {
        if maybe_line.is_some() { return; } // We only accept one line
        maybe_line = Some((line_ent, line.direction));
      }

      if let Some((line_ent, direction)) = maybe_line {
        let mut pts = vec![];
        for (point_ent, _, _) in (&*entities, &points, &selected).join() {
          pts.push(point_ent);
        }
        // Have to use a separate loop because `selected` was borrowed
        for point_ent in pts {

          // Check if there is already a parallel line
          let mut has_parallel = false;
          for desc in descendent_map.get_descendants(point_ent) {
            if let Some(line) = lines.get(*desc) {
              if line.direction == direction || line.direction == -direction {
                has_parallel = true;
              }
            }
          }

          // If does not have a parallel line, then create the parallel line
          if !has_parallel {
            let ent = entities.create();
            if let Err(err) = sym_lines.insert(ent, SymbolicLine::Parallel(line_ent, point_ent)) { panic!(err) };
            if let Err(err) = line_styles.insert(ent, LineStyle { color: Color::blue(), width: 2.0 }) { panic!(err) };
            if let Err(err) = selected.insert(ent, Selected) { panic!(err) };
            dirty_state.is_solver_dirty = true;
          }
        }
      }
    }
  }
}