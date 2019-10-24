use specs::prelude::*;
use geopad_core_lib::{utilities::*, components::screen_shapes::*, resources::*};
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
      // let mut closest_circles : Vec<(Entity, ScreenCircle)> = vec![];
      let mut maybe_smallest_dist_to_line : Option<f64> = None;
      let mut maybe_snap_point_on_line = None;
      // let mut maybe_smallest_dist_to_circle : Option<f64> = None;
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
          let closest_point = (*l).get_closest_point(mouse_pos);
          let dist = (closest_point - mouse_pos).magnitude();
          if dist <= SNAP_TO_POINT_THRES {
            closest_lines.push((entity, *l));
          }
          let norm_dist = dist / SNAP_TO_LINE_THRES;
          if norm_dist < 1.0 && !is_snapping_to_point {
            let t_of_point = (*l).t_of_point(closest_point);
            if maybe_smallest_dist_to_line.is_none() || norm_dist < maybe_smallest_dist_to_line.unwrap() {
              maybe_smallest_dist_to_line = Some(norm_dist);

              // Set the snap point to snap on line
              maybe_snap_point_on_line = Some(SnapPoint {
                position: closest_point,
                symbol: SnapPointType::SnapOnLine(entity, t_of_point.into()),
              });
            }
          }
        }
        // } else if let Some(c) = circles.get(entity) {
        //   let actual_circle = c.to_actual(&*vp);
        //   let actual_proj_point = actual_circle.center + (mouse_pos - actual_circle.center).normalized() * actual_circle.radius;
        //   let dist = (actual_proj_point - mouse_pos).magnitude();
        //   if dist <= SNAP_TO_CIRCLE_THRES {
        //     closest_circles.push((entity, *c));
        //   }
        //   let norm_dist = dist / SNAP_TO_CIRCLE_THRES;
        //   if norm_dist < 1.0 && !is_snapping_to_point {
        //     let virtual_proj_point = actual_proj_point.to_virtual(&*vp);
        //     let p_to_cen = virtual_proj_point - c.center;
        //     let theta = p_to_cen.y.atan2(p_to_cen.x);
        //     if maybe_smallest_dist_to_circle.is_none() || norm_dist < maybe_smallest_dist_to_circle.unwrap() {
        //       maybe_smallest_dist_to_circle = Some(norm_dist);
        //       maybe_snap_point_on_circle = Some(SnapPoint {
        //         position: virtual_proj_point,
        //         symbo: SnapPointType::SnapOnCircle(entity, theta),
        //       });
        //     }
        //   }
        // }
      }

      // Weight snap on point higher than snap on line
      if let Some(snap_point) = maybe_snap_point_on_point.or(maybe_snap_point_on_line).or(maybe_snap_point_on_circle) {
        maybe_snap_point.set(snap_point)
      }

      // // Check if snapping to an intersection
      // if !is_snapping_to_point {
      //   let mut maybe_smallest_dist = None;
      //   let mut has_line_line_itsct = false;

      //   // Line line intersection first
      //   for comb in closest_lines.iter().combinations(2) {
      //     if let &[(l1_ent, l1), (l2_ent, l2)] = &*comb {
      //       if let Some(itsct) = l1.intersect(*l2) {
      //         let actual : Vector2 = itsct.to_actual(&*vp);
      //         let norm_dist = (mouse_pos - actual).magnitude() / SNAP_TO_INTERSECTION_THRES;
      //         if norm_dist < 1.0 {
      //           if maybe_smallest_dist.is_none() || norm_dist < maybe_smallest_dist.unwrap() {
      //             maybe_smallest_dist = Some(norm_dist);

      //             // Set the snap point to intersection
      //             maybe_snap_point.set(SnapPoint {
      //               position: itsct,
      //               symbo: SnapPointType::SnapOnLineLineIntersection(*l1_ent, *l2_ent),
      //             });

      //             //
      //             has_line_line_itsct = true;
      //           }
      //         }
      //       }
      //     }
      //   }

      //   if !has_line_line_itsct {
      //     let mut has_circle_line_itsct = false;

      //     for ((line_ent, line), (circle_ent, circle)) in closest_lines.iter().cartesian_product(&closest_circles) {
      //       let ci = line.intersect(*circle);
      //       check_circle_intersection(&mouse_pos, &virtual_mouse_pos, &*vp, ci, maybe_smallest_dist.clone(), &mut |m| match m {
      //         Some((p, norm_dist, ty)) => {
      //           maybe_smallest_dist = Some(norm_dist);
      //           maybe_snap_point.set(SnapPoint {
      //             position: p,
      //             symbo: SnapPointType::SnapOnCircleLineIntersection(*circle_ent, *line_ent, ty),
      //           });
      //           has_circle_line_itsct = true;
      //         },
      //         None => (),
      //       });
      //     }

      //     if !has_circle_line_itsct {
      //       for comb in closest_circles.iter().combinations(2) {
      //         if let &[(c1_ent, c1), (c2_ent, c2)] = &*comb {
      //           check_circle_intersection(&mouse_pos, &virtual_mouse_pos, &*vp, c1.intersect(*c2), maybe_smallest_dist.clone(), &mut |m| match m {
      //             Some((p, norm_dist, ty)) => {
      //               maybe_smallest_dist = Some(norm_dist);
      //               maybe_snap_point.set(SnapPoint {
      //                 position: p,
      //                 symbo: SnapPointType::SnapOnCircleCircleIntersection(*c1_ent, *c2_ent, ty),
      //               });
      //             },
      //             None => (),
      //           });
      //         }
      //       }
      //     }
      //   }
      // }
    } else {
      maybe_snap_point.clear();
    }
  }
}

// fn check_circle_intersection<F>(
//   mouse_pos: &Vector2,
//   virtual_mouse_pos: &Vector2,
//   viewport: &Viewport,
//   ci: CircleIntersect,
//   maybe_smallest_dist: Option<f64>,
//   callback: &mut F,
// ) where F : FnMut(Option<(Vector2, f64, CircleIntersectionType)>) -> () {
//   match ci {
//     CircleIntersect::TwoPoints(p1, p2) => {
//       let (dist_1, dist_2) = ((p1 - *virtual_mouse_pos).magnitude(), (p2 - *virtual_mouse_pos).magnitude());
//       let (ty, p) = if dist_1 < dist_2 {
//         (CircleIntersectionType::First, p1)
//       } else {
//         (CircleIntersectionType::Second, p2)
//       };
//       let actual = p.to_actual(viewport);
//       let norm_dist = (*mouse_pos - actual).magnitude() / SNAP_TO_INTERSECTION_THRES;
//       if norm_dist < 1.0 {
//         if maybe_smallest_dist.is_none() || norm_dist < maybe_smallest_dist.unwrap() {
//           callback(Some((p, norm_dist, ty)));
//         }
//       }
//     },
//     CircleIntersect::OnePoint(p) => {
//       let actual = p.to_actual(viewport);
//       let norm_dist = (*mouse_pos - actual).magnitude() / SNAP_TO_INTERSECTION_THRES;
//       if norm_dist < 1.0 {
//         if maybe_smallest_dist.is_none() || norm_dist < maybe_smallest_dist.unwrap() {
//           callback(Some((p, norm_dist, CircleIntersectionType::First)));
//         }
//       }
//     },
//     CircleIntersect::None => ()
//   }
// }