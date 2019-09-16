use itertools::Itertools;
use specs::prelude::*;
use crate::{
  math::{Vector2, Intersect},
  util::Color,
  resources::{ToolState, InputState, Viewport, ViewportTransform, DirtyState, SpatialHashTable},
  components::{Selected, Point, PointStyle, SymbolicPoint, Line},
};

static SNAP_TO_POINT_THRES : f64 = 10.0; // In actual space
static SNAP_TO_LINE_THRES : f64 = 6.0; // In actual space
static SNAP_TO_INTERSECTION_THRES : f64 = 15.0; // In actual space

pub struct CreatePointSystem {
  hovering: Option<Entity>,
}

impl Default for CreatePointSystem {
  fn default() -> Self {
    Self { hovering: None }
  }
}

enum PointResult {
  AllowCreation { sym_point: SymbolicPoint, snap_point: Vector2 },
  JustSnapping { snap_point: Vector2 },
  Nothing,
}

impl<'a> System<'a> for CreatePointSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolState>,
    Read<'a, InputState>,
    Write<'a, DirtyState>,
    Read<'a, Viewport>,
    Read<'a, SpatialHashTable<Entity>>,
    ReadStorage<'a, Line>,
    WriteStorage<'a, Point>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    tool_state,
    input_state,
    mut dirty_state,
    vp,
    spatial_table,
    lines,
    mut points,
    mut sym_points,
    mut styles,
    mut selected,
  ): Self::SystemData) {

    // Only if input is dirty do we handle this event
    if !dirty_state.is_input_dirty { return; }

    // Make sure the tool state is currently at point state
    match *tool_state {
      ToolState::Point => {

        // First get the hovering point entity
        let hover_point = match self.hovering {
          Some(ent) => ent,
          None => {
            let ent = entities.create();
            self.hovering = Some(ent);
            if let Err(err) = points.insert(ent, input_state.mouse_abs_pos.to_virtual(&*vp)) { panic!(err); };
            if let Err(err) = styles.insert(ent, PointStyle { color: Color::new(1.0, 0.3, 0.3, 0.5), radius: 5. }) { panic!(err); };
            ent
          }
        };

        // Get initial mouse position
        let mouse_pos = input_state.mouse_abs_pos;
        let virtual_mouse_pos = input_state.mouse_abs_pos.to_virtual(&*vp);

        // Get all neghbors
        let maybe_neighbors = spatial_table.get_neighbor_entities(virtual_mouse_pos, &*vp);
        let mut result : PointResult = PointResult::Nothing;
        if let Some(neighbor_entities) = maybe_neighbors {

          // Keep the set of closest_lines for intersection computing
          let mut closest_lines : Vec<(Entity, Line)> = vec![];
          let mut maybe_smallest_dist : Option<f64> = None;
          let mut is_snapping_to_point = false;

          // Loop through entities
          for entity in neighbor_entities {
            if let Some(p) = points.get(entity) {
              let norm_dist = (p.to_actual(&*vp) - mouse_pos).magnitude() / SNAP_TO_POINT_THRES;
              if norm_dist < 1.0 {
                if let Some(smallest_dist) = maybe_smallest_dist {
                  if norm_dist < smallest_dist {
                    is_snapping_to_point = true;
                    maybe_smallest_dist = Some(norm_dist);
                    result = PointResult::JustSnapping { snap_point: *p };
                  }
                } else {
                  is_snapping_to_point = true;
                  maybe_smallest_dist = Some(norm_dist);
                  result = PointResult::JustSnapping { snap_point: *p };
                }
              }
            } else if let Some(l) = lines.get(entity) {
              let actual_proj_point = mouse_pos.project(l.to_actual(&*vp));
              let dist = (actual_proj_point - mouse_pos).magnitude();
              if dist <= SNAP_TO_POINT_THRES {
                closest_lines.push((entity, *l));
              }
              let norm_dist = dist / SNAP_TO_LINE_THRES;
              if norm_dist < 1.0 {
                if !is_snapping_to_point {
                  let virtual_proj_point = actual_proj_point.to_virtual(&*vp);
                  let p_to_origin = virtual_proj_point - l.origin;
                  let p_to_origin_dist = p_to_origin.magnitude();
                  let t = if p_to_origin.dot(l.direction) > 0.0 { p_to_origin_dist } else { -p_to_origin_dist };
                  if let Some(smallest_dist) = maybe_smallest_dist {
                    if norm_dist < smallest_dist {
                      maybe_smallest_dist = Some(norm_dist);
                      result = PointResult::AllowCreation {
                        sym_point: SymbolicPoint::OnLine(entity, t),
                        snap_point: virtual_proj_point,
                      };
                    }
                  } else {
                    maybe_smallest_dist = Some(norm_dist);
                    result = PointResult::AllowCreation {
                      sym_point: SymbolicPoint::OnLine(entity, t),
                      snap_point: virtual_proj_point,
                    };
                  }
                }
              }
            }
          }

          // Check if snapping to an intersection
          if !is_snapping_to_point {
            let mut maybe_smallest_dist = None;
            for comb in closest_lines.iter().combinations(2) {
              if let &[(l1_ent, l1), (l2_ent, l2)] = &*comb {
                if let Some(itsct) = l1.intersect(*l2) {
                  let actual : Vector2 = itsct.to_actual(&*vp);
                  let norm_dist = (mouse_pos - actual).magnitude() / SNAP_TO_INTERSECTION_THRES;
                  if norm_dist < 1.0 {
                    if let Some(smallest_dist) = maybe_smallest_dist {
                      if norm_dist < smallest_dist {
                        maybe_smallest_dist = Some(norm_dist);
                        result = PointResult::AllowCreation {
                          sym_point: SymbolicPoint::LineLineIntersect(*l1_ent, *l2_ent),
                          snap_point: itsct,
                        };
                      }
                    } else {
                      maybe_smallest_dist = Some(norm_dist);
                      result = PointResult::AllowCreation {
                        sym_point: SymbolicPoint::LineLineIntersect(*l1_ent, *l2_ent),
                        snap_point: itsct,
                      };
                    }
                  }
                }
              }
            }
          }
        }

        let snapping_style = PointStyle { color: Color::red(), radius: 6. }; // TODO: Make this lazy_static
        let regular_style = PointStyle { color: Color::new(1.0, 0.3, 0.3, 0.5), radius: 5. }; // TODO: Same
        let point_style = PointStyle { color: Color::red(), radius: 5. };
        let (hover_pos, hover_style) = match result {
          PointResult::AllowCreation { snap_point, .. } => (snap_point, snapping_style),
          PointResult::JustSnapping { snap_point } => (snap_point, snapping_style),
          PointResult::Nothing => (virtual_mouse_pos, regular_style),
        };
        if let Err(err) = points.insert(hover_point, hover_pos) { panic!(err); };
        if let Err(err) = styles.insert(hover_point, hover_style) { panic!(err); };

        if input_state.mouse_left_button.just_activated() {
          let maybe_sym_point_to_put = match result {
            PointResult::AllowCreation { sym_point, .. } => Some(sym_point),
            PointResult::Nothing => Some(SymbolicPoint::Free(virtual_mouse_pos)),
            _ => None
          };
          if let Some(sym_point) = maybe_sym_point_to_put {

            // Create the new point
            let new_point = entities.create();
            if let Err(err) = sym_points.insert(new_point, sym_point) { panic!(err) };
            if let Err(err) = styles.insert(new_point, point_style) { panic!(err) };
            if let Err(err) = selected.insert(new_point, Selected) { panic!(err) };

            // Set the solver to be dirty
            dirty_state.is_solver_dirty = true;
          }
        }
      },
      _ => { // If in other case, remove the hovering point
        match self.hovering {
          Some(ent) => {
            points.remove(ent);
            styles.remove(ent);
          },
          _ => (),
        }
      },
    }
  }
}