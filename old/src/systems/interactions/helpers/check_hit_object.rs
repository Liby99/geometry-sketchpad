use specs::prelude::*;
use crate::{
  utilities::{Vector2, Project},
  resources::{Viewport, ViewportTransform, SpatialHashTable},
  components::{Point, Line, Circle},
};

pub fn hitting_object<'a>(
  mouse_pos: Vector2,
  viewport: &Viewport,
  spatial_table: &SpatialHashTable<Entity>,
  points: &ReadStorage<'a, Point>,
  lines: &ReadStorage<'a, Line>,
  circles: &ReadStorage<'a, Circle>,
  threshold: f64,
) -> Option<Entity> {

  // First get the virtual mouse position
  let virtual_mouse_pos = mouse_pos.to_virtual(viewport);

  // Maybe selected...
  let mut maybe_selected_point : Option<(Entity, f64)> = None;
  let mut maybe_selected_line : Option<(Entity, f64)> = None;
  let mut maybe_selected_circle : Option<(Entity, f64)> = None;

  // Use spatial hash table to get potential neighbors
  let maybe_neighbors = spatial_table.get_neighbor_entities_of_point(virtual_mouse_pos, viewport);
  if let Some(neighbor_entities) = maybe_neighbors {
    for entity in neighbor_entities {
      if let Some(p) = points.get(entity) {
        let dist = (p.to_actual(viewport) - mouse_pos).magnitude();
        if dist < threshold && (maybe_selected_point.is_none() || dist < maybe_selected_point.unwrap().1) {
          maybe_selected_point = Some((entity, dist));
        }
      } else if let Some(l) = lines.get(entity) {
        let actual_line = l.to_actual(viewport);
        let actual_proj_point = mouse_pos.project(actual_line);
        if actual_line.point_is_on_line(actual_proj_point) {
          let dist = (actual_proj_point - mouse_pos).magnitude();
          if dist < threshold && (maybe_selected_line.is_none() || dist < maybe_selected_line.unwrap().1) {
            maybe_selected_line = Some((entity, dist));
          }
        }
      } else if let Some(c) = circles.get(entity) {
        let actual_circle = c.to_actual(viewport);
        let to_center_dist = (mouse_pos - actual_circle.center).magnitude();
        let dist = (to_center_dist - actual_circle.radius).abs();
        if dist < threshold && (maybe_selected_circle.is_none() || dist < maybe_selected_circle.unwrap().1) {
          maybe_selected_circle = Some((entity, dist));
        }
      }
    }
  }

  // Return point in priority to line
  maybe_selected_point.or(maybe_selected_line).or(maybe_selected_circle).map(|(ent, _)| ent)
}