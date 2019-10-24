use specs::prelude::*;
use geopad_core_lib::{resources::*, components::{styles::*, screen_shapes::*}};
use crate::{resources::*};

pub struct SnapCircleRenderer {
  snap_circle_entity: Option<Entity>,
}

impl Default for SnapCircleRenderer {
  fn default() -> Self {
    Self { snap_circle_entity: None }
  }
}

impl<'a> System<'a> for SnapCircleRenderer {
  type SystemData = (
    Entities<'a>,
    Read<'a, MaybeSnapPoint>,
    Read<'a, SnapCircle>,
    Read<'a, DefaultCircleStyle>,
    ReadStorage<'a, ScreenPoint>,
    WriteStorage<'a, ScreenCircle>,
    WriteStorage<'a, CircleStyle>,
  );

  fn run(&mut self, (
    entities,
    maybe_snap_point,
    snap_circle,
    default_circle_style,
    scrn_points,
    mut scrn_circles,
    mut circle_styles,
  ): Self::SystemData) {

    // First make sure we have an entity for rendering the snap point
    let ent = match self.snap_circle_entity {
      Some(ent) => ent,
      None => {
        let ent = entities.create();
        self.snap_circle_entity = Some(ent);
        ent
      },
    };

    // Then we render it when presented
    let mut draw = false;
    if let Some(first_point_ent) = snap_circle.maybe_first_point {
      if let Some(first_point_pos) = scrn_points.get(first_point_ent) {
        if let Some(SnapPoint { position: second_point_pos, .. }) = maybe_snap_point.get() {
          draw = true;

          let circle_style = default_circle_style.get().apply_alpha(0.6);
          let scrn_circle = ScreenCircle {
            center: *first_point_pos,
            radius: (second_point_pos - *first_point_pos).magnitude()
          };

          if let Err(err) = scrn_circles.insert(ent, scrn_circle) { panic!(err) }
          if let Err(err) = circle_styles.insert(ent, circle_style) { panic!(err) }
        }
      }
    }

    // If not draw then remove the snap circle
    if !draw {
      scrn_circles.remove(ent);
    }
  }
}