use specs::prelude::*;
use crate::{
  util::Color,
  resources::{ToolState, InputState, Viewport},
  components::point::{Point, PointStyle, SymbolicPoint},
};

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
      ToolState::Point => {
        let hover_point = match self.hovering {
          Some(ent) => ent,
          None => {
            let ent = entities.create();
            if let Err(err) = points.insert(ent, Point(vp.to_virtual(input_state.mouse_abs_pos))) { panic!(err); };
            if let Err(err) = styles.insert(ent, PointStyle { color: Color::new(1.0, 0.3, 0.3, 0.5), radius: 5. }) { panic!(err); };
            ent
          }
        };
        if let Err(err) = points.insert(hover_point, Point(vp.to_virtual(input_state.mouse_abs_pos))) { panic!(err); };

        // Only insert free point for now
        if input_state.mouse_left_button.just_activated() {
          let ent = entities.create();
          if let Err(err) = sym_points.insert(ent, SymbolicPoint::Free(vp.to_virtual(input_state.mouse_abs_pos))) { panic!(err); };
          if let Err(err) = styles.insert(ent, PointStyle { color: Color::red(), radius: 5. }) { panic!(err); };
        }
      },
      _ => (), // Do nothing otherwise
    }
  }
}