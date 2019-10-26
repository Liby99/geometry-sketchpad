use specs::prelude::*;
use core_lib::{utilities::*, components::{screen_shapes::*, styles::*}};

pub enum RenderUpdateEvent {
  InsertedPoint(Entity, ScreenPoint, PointStyle),
  InsertedLine(Entity, ScreenLine, LineStyle),
  InsertedCircle(Entity, ScreenCircle, CircleStyle),
  UpdatedPoint(Entity, ScreenPoint),
  UpdatedLine(Entity, ScreenLine),
  UpdatedCircle(Entity, ScreenCircle),
  UpdatedPointStyle(Entity, PointStyle),
  UpdatedLineStyle(Entity, LineStyle),
  UpdatedCircleStyle(Entity, CircleStyle),
  SelectedPoint(Entity),
  SelectedLine(Entity),
  SelectedCircle(Entity),
  DeselectedPoint(Entity),
  DeselectedLine(Entity),
  DeselectedCircle(Entity),
  RemovedEntity(Entity),
  None,
}