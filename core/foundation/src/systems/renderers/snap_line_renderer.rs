use specs::prelude::*;
use geopad_core_lib::{resources::*, components::{styles::*, screen_shapes::*}};
use crate::{resources::*};

pub struct SnapLineRenderer {
  snap_line_entity: Option<Entity>,
}

impl Default for SnapLineRenderer {
  fn default() -> Self {
    Self { snap_line_entity: None }
  }
}

impl<'a> System<'a> for SnapLineRenderer {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolState>,
    Read<'a, MaybeSnapPoint>,
    Read<'a, SnapLine>,
    Read<'a, DefaultLineStyle>,
    ReadStorage<'a, ScreenPoint>,
    WriteStorage<'a, ScreenLine>,
    WriteStorage<'a, LineStyle>,
  );

  fn run(&mut self, (
    entities,
    tool_state,
    maybe_snap_point,
    snap_line,
    default_line_style,
    scrn_points,
    mut scrn_lines,
    mut line_styles,
  ): Self::SystemData) {

    // First make sure we have an entity for rendering the snap point
    let ent = match self.snap_line_entity {
      Some(ent) => ent,
      None => {
        let ent = entities.create();
        self.snap_line_entity = Some(ent);
        ent
      },
    };

    // Then we render it when presented
    let mut draw = false;
    if let Some(first_point_ent) = snap_line.maybe_first_point {
      if let Some(first_point_pos) = scrn_points.get(first_point_ent) {
        if let Some(SnapPoint { position: second_point_pos, .. }) = maybe_snap_point.get() {
          if let Tool::Line(line_type) = tool_state.get() {
            draw = true;

            let line_style = default_line_style.get().apply_alpha(0.6);
            let scrn_line = ScreenLine { from: *first_point_pos, to: second_point_pos, line_type };

            if let Err(err) = scrn_lines.insert(ent, scrn_line) { panic!(err) }
            if let Err(err) = line_styles.insert(ent, line_style) { panic!(err) }
          }
        }
      }
    }

    // If not draw then remove the snap line
    if !draw {
      scrn_lines.remove(ent);
    }
  }
}