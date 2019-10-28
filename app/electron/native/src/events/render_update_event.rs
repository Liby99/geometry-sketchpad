use specs::prelude::*;
use core_lib::components::{screen_shapes::*, styles::*};

pub enum RenderUpdateEvent {
  None,
  InsertedPoint(Entity, ScreenPoint, PointStyle),
  InsertedLine(Entity, ScreenLine, LineStyle),
  InsertedCircle(Entity, ScreenCircle, CircleStyle),
  // InsertedRectangle(Entity, ScreenRectangle, RectangleStyle),
  UpdatedPoint(Entity, ScreenPoint),
  UpdatedLine(Entity, ScreenLine),
  UpdatedCircle(Entity, ScreenCircle),
  // InsertedRectangle(Entity, ScreenRectangle),
  UpdatedPointStyle(Entity, PointStyle),
  UpdatedLineStyle(Entity, LineStyle),
  UpdatedCircleStyle(Entity, CircleStyle),
  // UpdatedRectangleStyle(Entity, RectangleStyle),
  SelectedEntity(Entity),
  DeselectedEntity(Entity),
  RemovedEntity(Entity),
}

pub fn render_update_event_to_u32(event: &RenderUpdateEvent) -> u32 {
  match event {
    RenderUpdateEvent::None => 0,
    RenderUpdateEvent::InsertedPoint(_, _, _) => 1,
    RenderUpdateEvent::InsertedLine(_, _, _) => 2,
    RenderUpdateEvent::InsertedCircle(_, _, _) => 3,
    // RenderUpdateEvent::InsertedRectangle(_, _, _) => 4,
    RenderUpdateEvent::UpdatedPoint(_, _) => 5,
    RenderUpdateEvent::UpdatedLine(_, _) => 6,
    RenderUpdateEvent::UpdatedCircle(_, _) => 7,
    // RenderUpdateEvent::UpdatedRectangle(_, _) => 8,
    RenderUpdateEvent::UpdatedPointStyle(_, _) => 9,
    RenderUpdateEvent::UpdatedLineStyle(_, _) => 10,
    RenderUpdateEvent::UpdatedCircleStyle(_, _) => 11,
    // RenderUpdateEvent::UpdatedRectangleStyle(_, _) => 12,
    RenderUpdateEvent::RemovedEntity(_) => 13,
    RenderUpdateEvent::SelectedEntity(_) => 14,
    RenderUpdateEvent::DeselectedEntity(_) => 15,
  }
}