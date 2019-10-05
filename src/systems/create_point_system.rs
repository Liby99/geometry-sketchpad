use specs::prelude::*;
use shrev::EventChannel;
use crate::{
  util::Color,
  resources::{ToolState, InputState, MaybeSnapPoint, SnapPoint, SnapPointType, SketchEvent, Geometry, LastActivePoint},
  components::{SymbolicPoint, PointStyle, Selected},
};

pub struct CreatePointSystem;

impl<'a> System<'a> for CreatePointSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolState>,
    Read<'a, InputState>,
    Read<'a, MaybeSnapPoint>,
    Write<'a, EventChannel<SketchEvent>>,
    Write<'a, EventChannel<LastActivePoint>>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    tool_state,
    input_state,
    maybe_snap_point,
    mut sketch_events,
    mut last_active_point_event,
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
            SnapPointType::SnapOnPoint(entity) => {

              // If clicked on the snapped point, mark this point as last active
              last_active_point_event.single_write(LastActivePoint::new(entity));

              // Return none since we don't create new symbolic point
              None
            },
          };

          // Check if we need to create a point
          if let Some(sym_point) = symbolic_point {

            // First create the entity
            let entity = entities.create();
            if let Err(err) = sym_points.insert(entity, sym_point) { panic!(err) };
            if let Err(err) = styles.insert(entity, PointStyle { color: Color::red(), radius: 5. }) { panic!(err) };
            if let Err(err) = selected.insert(entity, Selected) { panic!(err) };

            // Then emit an event
            sketch_events.single_write(SketchEvent::Inserted(entity, Geometry::Point));

            // Mark this created entity as the last active point
            last_active_point_event.single_write(LastActivePoint::new(entity));
          }
        }
      }
    }
  }
}