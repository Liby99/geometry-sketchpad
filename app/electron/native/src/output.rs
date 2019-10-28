use specs::prelude::*;
use core_lib::components::{screen_shapes::*, styles::*};

pub enum RenderUpdateEvent {
  InsertedPoint(Entity, ScreenPoint, PointStyle),
  UpdatedPoint(Entity, ScreenPoint),
  // UpdatedLine(Entity, ScreenLine, LineStyle),
  // UpdatedCircle(Entity, ScreenCircle, CircleStyle),
  UpdatedPointStyle(Entity, PointStyle),
  SelectedEntity(Entity),
  DeselectedEntity(Entity),
  RemovedEntity(Entity),
  None,
}