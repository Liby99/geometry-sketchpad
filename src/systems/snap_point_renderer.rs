use specs::prelude::*;
use crate::{
  utilities::Color,
  resources::{MaybeSnapPoint, SnapPoint, SnapPointType},
  components::{Point, PointStyle}
};

pub struct SnapPointRenderer {
  point: Option<Entity>,
}

impl Default for SnapPointRenderer {
  fn default() -> Self {
    Self { point: None }
  }
}

impl<'a> System<'a> for SnapPointRenderer {
  type SystemData = (
    Entities<'a>,
    Read<'a, MaybeSnapPoint>,
    WriteStorage<'a, Point>,
    WriteStorage<'a, PointStyle>,
  );

  fn run(&mut self, (
    entities,
    maybe_snap_point,
    mut points,
    mut styles,
  ): Self::SystemData) {

    // First make sure that there's a point inside the system
    let ent = if let Some(ent) = self.point {
      ent
    } else {
      let ent = entities.create();
      self.point = Some(ent);
      ent
    };

    // Check if we need to render the snap point
    if let Some(SnapPoint { position, symbo }) = maybe_snap_point.get() {

      // First insert (update) the point position
      if let Err(err) = points.insert(ent, position) { panic!(err) };

      // Then insert the style.
      // If not snapped then it's slightly dimmer;
      // if snapped then it's larger and bright
      if let Err(err) = styles.insert(ent, match symbo {
        SnapPointType::NotSnapped => PointStyle { color: Color::new(1.0, 0.3, 0.3, 0.5), radius: 5. },
        _                         => PointStyle { color: Color::red(),                   radius: 6. },
      }) { panic!(err) };
    } else {
      points.remove(ent);
      styles.remove(ent);
    }
  }
}