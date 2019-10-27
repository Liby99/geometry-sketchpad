use specs::prelude::*;
use core_lib::components::{screen_shapes::*, styles::*};

pub enum RenderUpdateEvent {
  UpdatedPoint(Entity, ScreenPoint, PointStyle),
  // UpdatedLine(Entity, ScreenLine, LineStyle),
  // UpdatedCircle(Entity, ScreenCircle, CircleStyle),
  SelectedEntity(Entity),
  DeselectedEntity(Entity),
  RemovedEntity(Entity),
  None,
}