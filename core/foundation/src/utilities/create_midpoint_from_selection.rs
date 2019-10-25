use specs::prelude::*;
use geopad_core_lib::components::{symbolics::*, markers::*};

pub fn create_midpoint_from_selection<'a>(
  entities: &Entities<'a>,
  sym_points: &ReadStorage<'a, SymbolicPoint>,
  selecteds: &ReadStorage<'a, Selected>,
) -> Option<SymbolicPoint> {
  let (mut p1, mut p2) = (None, None);
  for (ent, _) in (entities, selecteds).join() {
    if sym_points.get(ent).is_some() {
      if p1.is_none() {
        p1 = Some(ent);
      } else if p2.is_none() {
        p2 = Some(ent);
      } else {
        return None;
      }
    } else {
      return None;
    }
  }
  match (p1, p2) {
    (Some(ent_1), Some(ent_2)) => Some(SymbolicPoint::MidPoint(ent_1, ent_2)),
    _ => None
  }
}