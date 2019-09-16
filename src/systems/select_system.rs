use specs::prelude::*;
use crate::{
  math::Vector2,
  resources::{Viewport, ViewportTransform, SpatialHashTable, InputState, ToolState},
  components::{Point, Line, Selected},
};

static SELECT_DIST_THRES : f64 = 5.0; // Pixel

pub struct SelectSystem;

impl<'a> System<'a> for SelectSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolState>,
    Read<'a, InputState>,
    Read<'a, Viewport>,
    Read<'a, SpatialHashTable<Entity>>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, Line>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    tool,
    input,
    vp,
    spatial_table,
    points,
    lines,
    mut selected,
  ): Self::SystemData) {
    match *tool {
      ToolState::Select => {
        if input.mouse_left_button.just_activated() {
          let mouse_pos = input.mouse_abs_pos;
          let virtual_mouse_pos = input.mouse_abs_pos.to_virtual(&*vp);

          let maybe_neighbors = spatial_table.get_neighbor_entities(virtual_mouse_pos, &*vp);
          let mut maybe_selected_point : Option<(Entity, f64)> = None;
          let mut maybe_selected_line : Option<(Entity, f64)> = None;
          if let Some(neighbor_entities) = maybe_neighbors {
            for entity in neighbor_entities {
              if let Some(p) = points.get(entity) {
                let dist = (p.to_actual(&*vp) - mouse_pos).magnitude();
                if dist < SELECT_DIST_THRES && (maybe_selected_point.is_none() || dist < maybe_selected_point.unwrap().1) {
                  maybe_selected_point = Some((entity, dist));
                }
              } else if let Some(l) = lines.get(entity) {
                let actual_proj_point = mouse_pos.project(l.to_actual(&*vp));
                let dist = (actual_proj_point - mouse_pos).magnitude();
                if dist < SELECT_DIST_THRES && (maybe_selected_line.is_none() || dist < maybe_selected_line.unwrap().1) {
                  maybe_selected_line = Some((entity, dist));
                }
              }
            }
          }

          if let Some((ent, _)) = maybe_selected_point.or(maybe_selected_line) {

            // Select or deselect based on its select state
            match selected.get(ent) {
              Some(_) => { selected.remove(ent); },
              None => if let Err(err) = selected.insert(ent, Selected) { panic!(err); },
            }
          } else {
            // If nothing selected, clear the selection
            selected.clear();
          }

          // // TODO: CHange this logic to getting the closest point & Make this point size dependent
          // for (ent, p) in (&entities, &points).join() {
          //   if (Vector2::from(p.to_actual(&*vp)) - mouse_pos).magnitude() <= SELECT_DIST_THRES {
          //     match selected.get(ent) {
          //       Some(_) => { selected.remove(ent); },
          //       None => if let Err(err) = selected.insert(ent, Selected) {
          //         panic!("Error selecting {:?}: {}", ent, err);
          //       },
          //     }
          //     return;
          //   }
          // }
        }
      },
      _ => (),
    }
  }
}