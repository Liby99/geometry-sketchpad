use specs::prelude::*;
use crate::{
  utilities::Color,
  resources::{
    ToolState, Tool,
    geometry::{CreateLineData, SnapPoint, MaybeSnapPoint},
  },
  components::{Point, Circle, CircleStyle},
};

pub struct CreateCircleRenderer {
  entity: Option<Entity>,
}

impl Default for CreateCircleRenderer {
  fn default() -> Self {
    Self { entity: None }
  }
}

impl<'a> System<'a> for CreateCircleRenderer {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolState>,
    Read<'a, CreateLineData>,
    Read<'a, MaybeSnapPoint>,
    ReadStorage<'a, Point>,
    WriteStorage<'a, Circle>,
    WriteStorage<'a, CircleStyle>,
  );

  fn run(&mut self, (
    entities,
    tool_state,
    create_line_data,
    maybe_snap_point,
    points,
    mut circles,
    mut styles,
  ): Self::SystemData) {

    // First make sure there's an entity here
    let ent = if let Some(ent) = self.entity { ent } else {
      let ent = entities.create();
      self.entity = Some(ent);
      ent
    };

    // Check if it is circle tool
    if tool_state.get() == Tool::Circle {

      // Do caching
      let mut need_render = false;

      // Then check if we have the first point
      if let Some(first_point_entity) = create_line_data.maybe_first_point {
        if let Some(first_point_position) = points.get(first_point_entity) {
          if let Some(SnapPoint { position: second_point_position, .. }) = maybe_snap_point.get() {

            // Need to make sure that the first point is not second point
            if *first_point_position != second_point_position {
              let radius = (second_point_position - *first_point_position).magnitude();

              // Insert line and line styles
              need_render = true;
              if let Err(err) = circles.insert(ent, Circle::new(*first_point_position, radius)) { panic!(err) }
              if let Err(err) = styles.insert(ent, CircleStyle { color: Color::new(0.3, 0.6, 0.3, 0.5), width: 2. }) { panic!(err) }
            }
          }
        }
      }

      if !need_render {
        circles.remove(ent);
        styles.remove(ent);
      }
    }
  }
}