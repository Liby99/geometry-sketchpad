use specs::prelude::*;
use geopad_core_lib::{resources::*, components::{styles::*, screen_shapes::*}};
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
    let ent = match self.snap_point_entity {
      Some(ent) => ent,
      None => {
        let ent = entities.create();
        self.snap_point_entity = Some(ent);
        ent
      },
    };

    if let Some(snap_point) = maybe_snap_point.get() {

      // First generate the point style of the snap point
      let point_style = match snap_point.symbol {
        SnapPointType::NotSnapped => default_point_style.get().apply_alpha(0.6),
        _ => default_point_style.get().resize(3.0),
      };

      // Then insert the components
      if let Err(err) = scrn_points.insert(ent, snap_point.position) { panic!(err) }
      if let Err(err) = point_styles.insert(ent, point_style) { panic!(err) }
    } else {
      scrn_points.remove(ent);
    }
  }
}