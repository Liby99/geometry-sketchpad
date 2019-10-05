use specs::prelude::*;
use crate::{
  util::Color,
  resources::{CreateLineData, SnapPoint, MaybeSnapPoint},
  components::{Point, Line, LineStyle},
};

pub struct CreateLineRenderer {
  entity: Option<Entity>,
}

impl Default for CreateLineRenderer {
  fn default() -> Self {
    Self { entity: None }
  }
}

impl<'a> System<'a> for CreateLineRenderer {
  type SystemData = (
    Entities<'a>,
    Read<'a, CreateLineData>,
    Read<'a, MaybeSnapPoint>,
    ReadStorage<'a, Point>,
    WriteStorage<'a, Line>,
    WriteStorage<'a, LineStyle>,
  );

  fn run(&mut self, (
    entities,
    create_line_data,
    maybe_snap_point,
    points,
    mut lines,
    mut styles
  ): Self::SystemData) {

    // First make sure there's an entity here
    let ent = if let Some(ent) = self.entity { ent } else {
      let ent = entities.create();
      self.entity = Some(ent);
      ent
    };

    // Do caching
    let mut need_render = false;

    // Then check if we have the first point
    if let Some(first_point_entity) = create_line_data.maybe_first_point {
      if let Some(first_point_position) = points.get(first_point_entity) {
        if let Some(SnapPoint { position: second_point_position, .. }) = maybe_snap_point.get() {

          // Need to make sure that the first point is not second point
          if *first_point_position != second_point_position {

            // Insert line and line styles
            need_render = true;
            if let Err(err) = lines.insert(ent, Line::from_to(*first_point_position, second_point_position)) { panic!(err) }
            if let Err(err) = styles.insert(ent, LineStyle { color: Color::new(0.3, 0.3, 1.0, 0.5), width: 2. }) { panic!(err) }
          }
        }
      } else {
        panic!("[create_line_renderer] First point position does not exist");
      }
    }

    if !need_render {
      lines.remove(ent);
      styles.remove(ent);
    }
  }
}