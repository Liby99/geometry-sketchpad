use itertools::Itertools;
use specs::prelude::*;
use geopad_core_lib::{math::*, utilities::*, components::{symbolics::*, screen_shapes::*}, resources::*};
use crate::{resources::*};

// In actual space
static SNAP_TO_POINT_THRES : ScreenScalar = ScreenScalar(12.0);
static SNAP_TO_LINE_THRES : ScreenScalar = ScreenScalar(8.0);
static SNAP_TO_CIRCLE_THRES : ScreenScalar = ScreenScalar(8.0);
static SNAP_TO_INTERSECTION_THRES : ScreenScalar = ScreenScalar(15.0);

#[derive(Default)]
pub struct SnapPointViaMouse;

impl<'a> System<'a> for SnapPointViaMouse {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, ToolState>,
    Read<'a, SpatialEntityMap>,
    Write<'a, MaybeSnapPoint>,
    ReadStorage<'a, ScreenPoint>,
    ReadStorage<'a, ScreenLine>,
    ReadStorage<'a, ScreenCircle>,
  );

  fn run(&mut self, (
    input_state,
    tool_state,
    spatial_entity_map,
    mut maybe_snap_point,
    scrn_points,
    scrn_lines,
    scrn_circles,
  ): Self::SystemData) {
    if tool_state.need_snap_point() {
      let mouse_pos = input_state.mouse_abs_pos;

      // Set the snap point to free point as a default case
      maybe_snap_point.set(SnapPoint {
        position: mouse_pos,
        symbol: SnapPointType::NotSnapped,
      });

      // Then get the potential neighbors
      let neighbor_entities = spatial_entity_map.get_entities_near_point(mouse_pos.into(), SNAP_TO_POINT_THRES.into());

      let mut maybe_smallest_dist_to_point : Option<f64> = None;
      let mut maybe_snap_point_on_point = None;
      let mut is_snapping_to_point = false;
      let mut closest_lines : Vec<(Entity, ScreenLine)> = vec![];
      let mut closest_circles : Vec<(Entity, ScreenCircle)> = vec![];
      let mut maybe_smallest_dist_to_line : Option<f64> = None;
      let mut maybe_snap_point_on_line = None;
      let mut maybe_smallest_dist_to_circle : Option<f64> = None;
      let mut maybe_snap_point_on_circle = None;

      // Loop through all the neighbor entities
      for entity in neighbor_entities {
        if let Some(p) = scrn_points.get(entity) {
          let norm_dist = (*p - mouse_pos).magnitude() / SNAP_TO_POINT_THRES;
          if norm_dist < 1.0 {
            if maybe_smallest_dist_to_point.is_none() || norm_dist < maybe_smallest_dist_to_point.unwrap() {
              is_snapping_to_point = true;
              maybe_smallest_dist_to_point = Some(norm_dist);

              // Set the snap point to snap on point
              maybe_snap_point_on_point = Some(SnapPoint {
                position: *p,
                symbol: SnapPointType::SnapOnPoint(entity)
              });
            }
          }
        } else if let Some(l) = scrn_lines.get(entity) {
          let l = *l;
          let closest_point = l.get_closest_point(mouse_pos);
          let dist = (closest_point - mouse_pos).magnitude();
          if dist <= SNAP_TO_POINT_THRES {
            closest_lines.push((entity, l));
          }
          let norm_dist = dist / SNAP_TO_LINE_THRES;
          if norm_dist < 1.0 && !is_snapping_to_point {
            let t = l.t_of_point(closest_point) / l.from_to_length();
            if maybe_smallest_dist_to_line.is_none() || norm_dist < maybe_smallest_dist_to_line.unwrap() {
              maybe_smallest_dist_to_line = Some(norm_dist);

              // Set the snap point to snap on line
              maybe_snap_point_on_line = Some(SnapPoint {
                position: closest_point,
                symbol: SnapPointType::SnapOnLine(entity, t),
              });
            }
          }
        } else if let Some(c) = scrn_circles.get(entity) {
          let c = *c;
          let proj_point = mouse_pos.project(c);
          let dist = (proj_point - mouse_pos).magnitude();
          if dist <= SNAP_TO_CIRCLE_THRES {
            closest_circles.push((entity, c));
          }
          let norm_dist = dist / SNAP_TO_CIRCLE_THRES;
          if norm_dist < 1.0 && !is_snapping_to_point {
            let p_to_cen : Vector2 = (proj_point - c.center).into();
            let theta = -p_to_cen.y.atan2(p_to_cen.x);
            if maybe_smallest_dist_to_circle.is_none() || norm_dist < maybe_smallest_dist_to_circle.unwrap() {
              maybe_smallest_dist_to_circle = Some(norm_dist);
              maybe_snap_point_on_circle = Some(SnapPoint {
                position: proj_point,
                symbol: SnapPointType::SnapOnCircle(entity, theta),
              });
            }
          }
        }
      }

      // Weight snap on point higher than snap on line
      if let Some(snap_point) = maybe_snap_point_on_point.or(maybe_snap_point_on_line).or(maybe_snap_point_on_circle) {
        maybe_snap_point.set(snap_point)
      }

      // Check if snapping to an intersection
      if !is_snapping_to_point {
        let mut maybe_smallest_dist = None;
        let mut has_line_line_itsct = false;

        // Line line intersection first
        for comb in closest_lines.iter().combinations(2) {
          if let &[(l1_ent, l1), (l2_ent, l2)] = &*comb {
            let l1 = *l1;
            let l2 = *l2;
            if let Some(itsct) = l1.intersect(l2) {
              let norm_dist = (mouse_pos - itsct).magnitude() / SNAP_TO_INTERSECTION_THRES;
              if norm_dist < 1.0 {
                if maybe_smallest_dist.is_none() || norm_dist < maybe_smallest_dist.unwrap() {
                  maybe_smallest_dist = Some(norm_dist);

                  // Set the snap point to intersection
                  maybe_snap_point.set(SnapPoint {
                    position: itsct,
                    symbol: SnapPointType::SnapOnLineLineIntersection(*l1_ent, *l2_ent),
                  });

                  has_line_line_itsct = true;
                }
              }
            }
          }
        }

        if !has_line_line_itsct {
          let mut has_circle_line_itsct = false;

          for ((line_ent, line), (circle_ent, circle)) in closest_lines.iter().cartesian_product(&closest_circles) {
            let ci = line.intersect(*circle);
            check_circle_intersection(mouse_pos, ci, maybe_smallest_dist.clone(), &mut |m| match m {
              Some((p, norm_dist, ty)) => {
                maybe_smallest_dist = Some(norm_dist);
                maybe_snap_point.set(SnapPoint {
                  position: p,
                  symbol: SnapPointType::SnapOnCircleLineIntersection(*circle_ent, *line_ent, ty),
                });
                has_circle_line_itsct = true;
              },
              None => (),
            });
          }

          if !has_circle_line_itsct {
            for comb in closest_circles.iter().combinations(2) {
              if let &[(c1_ent, c1), (c2_ent, c2)] = &*comb {

                // Note: here we use `c1.intersect(c2).reverse` because we are doing computation in
                // screen space. For circle intersection we have to order circle sequentially, and the order
                // is reversed between screen space and virtual space. So in order to get a correct virtual
                // space answer, we reverse the result
                check_circle_intersection(mouse_pos, c1.intersect(*c2).reverse(), maybe_smallest_dist.clone(), &mut |m| match m {
                  Some((p, norm_dist, ty)) => {
                    maybe_smallest_dist = Some(norm_dist);
                    maybe_snap_point.set(SnapPoint {
                      position: p,
                      symbol: SnapPointType::SnapOnCircleCircleIntersection(*c1_ent, *c2_ent, ty),
                    });
                  },
                  None => (),
                });
              }
            }
          }
        }
      }
    } else {
      maybe_snap_point.clear();
    }
  }
}

fn check_circle_intersection<F>(
  mouse_pos: ScreenPosition,
  ci: ScreenCircleIntersect,
  maybe_smallest_dist: Option<f64>,
  callback: &mut F,
) where F : FnMut(Option<(ScreenPosition, f64, CircleIntersectId)>) -> () {
  match ci {
    ScreenCircleIntersect::TwoPoints(p1, p2) => {
      let (dist_1, dist_2) = ((p1 - mouse_pos).magnitude(), (p2 - mouse_pos).magnitude());
      let (ty, p) = if dist_1 < dist_2 {
        (CircleIntersectId::First, p1)
      } else {
        (CircleIntersectId::Second, p2)
      };
      let norm_dist = (mouse_pos - p).magnitude() / SNAP_TO_INTERSECTION_THRES;
      if norm_dist < 1.0 {
        if maybe_smallest_dist.is_none() || norm_dist < maybe_smallest_dist.unwrap() {
          callback(Some((p, norm_dist, ty)));
        }
      }
    },
    ScreenCircleIntersect::OnePoint(p) => {
      let norm_dist = (mouse_pos - p).magnitude() / SNAP_TO_INTERSECTION_THRES;
      if norm_dist < 1.0 {
        if maybe_smallest_dist.is_none() || norm_dist < maybe_smallest_dist.unwrap() {
          callback(Some((p, norm_dist, CircleIntersectId::First)));
        }
      }
    },
    ScreenCircleIntersect::None => ()
  }
}