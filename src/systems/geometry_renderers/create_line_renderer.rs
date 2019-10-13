use specs::prelude::*;
use crate::{
  utilities::{Color, LineType},
  resources::{
    ToolState, Tool, LineTool,
    geometry::{CreateLineData, SnapPoint, MaybeSnapPoint},
  },
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

/// # Create Line Renderer
///
/// This `CreateLineRenderer` system intends to render a dimmed line when the first
/// point is created and before placing the second point. Given the first point,
/// it will take the second point from the `SnapPoint`. It will only show line when
/// 1. It's currently Line Tool
impl<'a> System<'a> for CreateLineRenderer {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolState>,
    Read<'a, CreateLineData>,
    Read<'a, MaybeSnapPoint>,
    ReadStorage<'a, Point>,
    WriteStorage<'a, Line>,
    WriteStorage<'a, LineStyle>,
  );

  fn run(&mut self, (
    entities,
    tool_state,
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

    // Check if it is line tool
    if let Tool::Line(line_tool) = tool_state.get() {

      // Then check if we have the first point
      if let Some(first_point_entity) = create_line_data.maybe_first_point {
        if let Some(first_point_position) = points.get(first_point_entity) {
          if let Some(SnapPoint { position: second_point_position, .. }) = maybe_snap_point.get() {

            // Need to make sure that the first point is not second point
            if *first_point_position != second_point_position {
              let origin = *first_point_position;
              let diff = second_point_position - origin;
              let direction = diff.normalized();
              let line = match line_tool {
                LineTool::Line => Line { origin, direction, ..Default::default() },
                LineTool::Ray => Line { origin, direction, line_type: LineType::Ray },
                LineTool::Segment => Line { origin, direction, line_type: LineType::Segment(diff.magnitude()) }
              };

              // Insert line and line styles
              need_render = true;
              if let Err(err) = lines.insert(ent, line) { panic!(err) }
              if let Err(err) = styles.insert(ent, LineStyle { color: Color::new(0.3, 0.3, 1.0, 0.5), width: 2. }) { panic!(err) }
            }
          }
        }
      }
    }

    if !need_render {
      lines.remove(ent);
      styles.remove(ent);
    }
  }
}