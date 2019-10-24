use specs::prelude::*;
use geopad_core_lib::{math::*, resources::*, utilities::*, components::screen_shapes::*};

pub fn hitting_object<'a>(
  mouse_pos: ScreenPosition,
  spatial_entity_map: &SpatialEntityMap,
  scrn_points: &ReadStorage<'a, ScreenPoint>,
  scrn_lines: &ReadStorage<'a, ScreenLine>,
  scrn_circles: &ReadStorage<'a, ScreenCircle>,
  threshold: ScreenScalar,
) -> Option<Entity> {

  // Maybe selected...
  let mut maybe_selected_point : Option<(Entity, ScreenScalar)> = None;
  let mut maybe_selected_line : Option<(Entity, ScreenScalar)> = None;
  let mut maybe_selected_circle : Option<(Entity, ScreenScalar)> = None;

  // Use spatial hash table to get potential neighbors
  let neighbor_entities = spatial_entity_map.get_entities_near_point(mouse_pos.into(), threshold.into());
  for entity in neighbor_entities {
    if let Some(p) = scrn_points.get(entity) {
      let dist = (*p - mouse_pos).magnitude();
      if dist < threshold && (maybe_selected_point.is_none() || dist < maybe_selected_point.unwrap().1) {
        maybe_selected_point = Some((entity, dist));
      }
    } else if let Some(l) = scrn_lines.get(entity) {
      let proj_point = l.get_closest_point(mouse_pos);
      let dist = (proj_point - mouse_pos).magnitude();
      if dist < threshold && (maybe_selected_line.is_none() || dist < maybe_selected_line.unwrap().1) {
        maybe_selected_line = Some((entity, dist));
      }
    } else if let Some(c) = scrn_circles.get(entity) {
      let dist = (mouse_pos - mouse_pos.project(*c)).magnitude();
      if dist < threshold && (maybe_selected_circle.is_none() || dist < maybe_selected_circle.unwrap().1) {
        maybe_selected_circle = Some((entity, dist));
      }
    }
  }

  // Return point in priority to line
  maybe_selected_point.or(maybe_selected_line).or(maybe_selected_circle).map(|(ent, _)| ent)
}