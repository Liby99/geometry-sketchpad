use specs::prelude::*;
use geopad_core_lib::components::{symbolics::*, markers::*};

/// We can have, in selection, a single line, and lots of points
pub fn check_perp_para_selection<'a>(
  entities: &Entities<'a>,
  sym_points: &ReadStorage<'a, SymbolicPoint>,
  sym_lines: &ReadStorage<'a, SymbolicLine>,
  selecteds: &ReadStorage<'a, Selected>,
) -> Option<(Entity, Vec<Entity>)> {
  let mut maybe_line_ent = None;
  let mut point_ents = Vec::new();
  for (entity, _) in (entities, selecteds).join() {
    if sym_lines.get(entity).is_some() {
      if maybe_line_ent.is_none() {
        maybe_line_ent = Some(entity);
      } else {
        return None;
      }
    } else if sym_points.get(entity).is_some() {
      point_ents.push(entity);
    }
  }
  if let Some(line_ent) = maybe_line_ent {
    if !point_ents.is_empty() {
      Some((line_ent, point_ents))
    } else {
      None
    }
  } else {
    None
  }
}