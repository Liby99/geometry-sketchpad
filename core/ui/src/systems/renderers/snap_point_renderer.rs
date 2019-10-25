use specs::prelude::*;
use core_lib::{resources::*, components::{styles::*, screen_shapes::*}};
use crate::{resources::*};

pub struct SnapPointRenderer {
  snap_point_entity: Option<Entity>,
}

impl Default for SnapPointRenderer {
  fn default() -> Self {
    Self { snap_point_entity: None }
  }
}

impl<'a> System<'a> for SnapPointRenderer {
  type SystemData = (
    Entities<'a>,
    Read<'a, MaybeSnapPoint>,
    Read<'a, DefaultPointStyle>,
    WriteStorage<'a, ScreenPoint>,
    WriteStorage<'a, PointStyle>,
  );

  fn run(&mut self, (
    entities,
    maybe_snap_point,
    default_point_style,
    mut scrn_points,
    mut point_styles,
  ): Self::SystemData) {

    // First make sure we have an entity for rendering the snap point
    let ent = match self.snap_point_entity {
      Some(ent) => ent,
      None => {
        let ent = entities.create();
        self.snap_point_entity = Some(ent);
        ent
      },
    };

    // Then we render it
    if let Some(snap_point) = maybe_snap_point.get() {

      // First generate the point style of the snap point
      let point_style = match snap_point.symbol {

        // For not snapped, we want dimmed style
        SnapPointType::NotSnapped => default_point_style.get().apply_alpha(0.6),

        // For snapped, we want it to be bigger than the default
        _ => default_point_style.get().resize(1.0),
      };

      // Then insert the components
      if let Err(err) = scrn_points.insert(ent, snap_point.position) { panic!(err) }
      if let Err(err) = point_styles.insert(ent, point_style) { panic!(err) }
    } else {
      scrn_points.remove(ent);
    }
  }
}