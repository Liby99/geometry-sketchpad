use specs::prelude::*;
use crate::{
  math::Vector2,
  util::Color,
  resources::{ToolState, InputState, Viewport},
  components::point::{Point, PointStyle, SymbolicPoint},
};

static SNAP_TO_POINT_THRES : f64 = 10.0; // In actual space

pub struct CreatePointSystem {
  hovering: Option<Entity>,
}

impl Default for CreatePointSystem {
  fn default() -> Self {
    Self { hovering: None }
  }
}

impl<'a> System<'a> for CreatePointSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolState>,
    Read<'a, InputState>,
    Read<'a, Viewport>,
    WriteStorage<'a, Point>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, SymbolicPoint>,
  );

  fn run(&mut self, (
    entities,
    tool_state,
    input_state,
    vp,
    mut points,
    mut styles,
    mut sym_points
  ): Self::SystemData) {
    match *tool_state {
      ToolState::Point => { // Make sure the tool state is currently at point state

        // First get the hovering point entity
        let hover_point = match self.hovering {
          Some(ent) => ent,
          None => {
            let ent = entities.create();
            if let Err(err) = points.insert(ent, Point(vp.to_virtual(input_state.mouse_abs_pos))) { panic!(err); };
            if let Err(err) = styles.insert(ent, PointStyle { color: Color::new(1.0, 0.3, 0.3, 0.5), radius: 5. }) { panic!(err); };
            ent
          }
        };

        // Get initial mouse position
        let mouse_pos = Vector2::from(input_state.mouse_abs_pos);
        let virtual_mouse_pos = vp.to_virtual(input_state.mouse_abs_pos);

        // Then calculate the closest point this point should snap to
        let mut closest_point = None;
        for (_, Point(p)) in (&sym_points, &points).join() { // Only snap to points with sym_points
          let dist = (Vector2::from(vp.to_actual(*p)) - mouse_pos).magnitude();
          if dist <= SNAP_TO_POINT_THRES {
            closest_point = match closest_point {
              Some((_, d)) => if dist < d { Some((p, dist)) } else { closest_point },
              None => Some((p, dist))
            };
          }
        }

        // Calculate the final position
        let snapping_to_point = closest_point.is_some();
        let final_pos = if let Some((p, _)) = closest_point { *p } else { virtual_mouse_pos };
        if let Err(err) = points.insert(hover_point, Point(final_pos)) { panic!(err); };

        // Change the style if snapping
        let snapping = snapping_to_point;
        if snapping {
          if let Err(err) = styles.insert(hover_point, PointStyle { color: Color::red(), radius: 6. }) { panic!(err); };
        } else {
          if let Err(err) = styles.insert(hover_point, PointStyle { color: Color::red(), radius: 5. }) { panic!(err); };
        }

        // Only insert free point for now
        if input_state.mouse_left_button.just_activated() {
          if !snapping_to_point {
            let ent = entities.create();
            if let Err(err) = sym_points.insert(ent, SymbolicPoint::Free(vp.to_virtual(input_state.mouse_abs_pos))) { panic!(err); };
            if let Err(err) = styles.insert(ent, PointStyle { color: Color::red(), radius: 5. }) { panic!(err); };
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