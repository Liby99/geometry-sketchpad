use specs::prelude::*;
use crate::{
  util::Color,
  resources::{ToolState, InputState, MaybeSnapPoint, SnapPoint, SnapPointType},
  components::{SymbolicPoint, PointStyle, Selected},
};

pub struct CreatePointSystem;

impl<'a> System<'a> for CreatePointSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolState>,
    Read<'a, InputState>,
    Read<'a, MaybeSnapPoint>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    tool_state,
    input_state,
    maybe_snap_point,
    mut sym_points,
    mut styles,
    mut selected,
  ): Self::SystemData) {
    if tool_state.depend_on_active_point() {
      if input_state.mouse_left_button.just_activated() {
        if let Some(SnapPoint { position, symbo }) = maybe_snap_point.get() {

          // Get the symbolic point data from symbo
          let symbolic_point = match symbo {
            SnapPointType::NotSnapped => Some(SymbolicPoint::Free(position)),
            SnapPointType::SnapOnLine(line_ent, t) => Some(SymbolicPoint::OnLine(line_ent, t)),
            SnapPointType::SnapOnIntersection(l1_ent, l2_ent) => Some(SymbolicPoint::LineLineIntersect(l1_ent, l2_ent)),
            _ => None
          };

          // Check if we need to create a point
          if let Some(sym_point) = symbolic_point {

            // First create the entity
            let ent = entities.create();
            if let Err(err) = sym_points.insert(ent, sym_point) { panic!(err) };
            if let Err(err) = styles.insert(ent, PointStyle { color: Color::red(), radius: 5. }) { panic!(err) };
            if let Err(err) = selected.insert(ent, Selected) { panic!(err) };

            // Then emit an event
            // events.push(InsertPoint(ent));
          }
        }
      }
    }
  }
}