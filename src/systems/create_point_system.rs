use specs::prelude::*;
use crate::{
  math::Vector2,
  util::Color,
  resources::{ToolState, InputState, Viewport},
  components::{
    selected::Selected,
    point::{Point, PointStyle, SymbolicPoint},
    line::Line,
  },
};

static SNAP_TO_POINT_THRES : f64 = 10.0; // In actual space
static SNAP_TO_LINE_THRES : f64 = 6.0; // In actual space

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
    vp,
    lines,
    mut points,
    mut sym_points,
    mut styles,
    mut selected,
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
              Some((_, d)) => if dist < d { Some((*p, dist)) } else { closest_point },
              None => Some((*p, dist))
            };
          }
        }

        // Check if is snapping to point
        let snapping_to_point = closest_point.is_some();
        if let Some((p, _)) = closest_point {
          if let Err(err) = points.insert(hover_point, Point(p)) { panic!(err); };
        }

        // Check if snapping to a line
        let mut closest_line : Option<(Vector2, Entity, f64, f64)> = None; // (closest_point, line_ent, t, dist_to_line)
        if !snapping_to_point {
          for (ent, Line { origin, direction }) in (&*entities, &lines).join() {
            let actual_line = Line { origin: Vector2::from(vp.to_actual(*origin)), direction: vec2![direction.x, -direction.y] };
            let closest_point = mouse_pos.project(actual_line);
            let dist = (mouse_pos - closest_point).magnitude();
            if dist <= SNAP_TO_LINE_THRES {
              let virtual_closest_point = vp.to_virtual(closest_point.into());
              let t = (virtual_closest_point - *origin).magnitude();
              closest_line = match closest_line {
                Some((_, _, _, d)) => if dist < d { Some((virtual_closest_point, ent, t, dist)) } else { closest_line },
                None => Some((virtual_closest_point, ent, t, dist))
              };
            }
          }
        }

        // If snapping to line, then put the hover_point on the Point
        let snapping_to_line = closest_line.is_some();
        if let Some((p, _, _, _)) = closest_line {
          if let Err(err) = points.insert(hover_point, Point(p)) { panic!(err); };
        }

        // Change the style if snapping. If so then update the style. Else restore the style
        let snapping = snapping_to_point || snapping_to_line;
        if snapping {
          if let Err(err) = styles.insert(hover_point, PointStyle { color: Color::red(), radius: 6. }) { panic!(err); };
        } else {
          if let Err(err) = points.insert(hover_point, Point(virtual_mouse_pos)) { panic!(err); };
          if let Err(err) = styles.insert(hover_point, PointStyle { color: Color::new(1.0, 0.3, 0.3, 0.5), radius: 5. }) { panic!(err); };
        }

        // Only insert free point for now
        if input_state.mouse_left_button.just_activated() {
          if !snapping_to_point {
            let ent = entities.create();
            let sym_point = if let Some((_, line_ent, t, _)) = closest_line {
              SymbolicPoint::OnLine(line_ent, t)
            } else {
              SymbolicPoint::Free(virtual_mouse_pos)
            };
            if let Err(err) = sym_points.insert(ent, sym_point) { panic!(err); };
            if let Err(err) = styles.insert(ent, PointStyle { color: Color::red(), radius: 5. }) { panic!(err); };
            if let Err(err) = selected.insert(ent, Selected) { panic!(err); };
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