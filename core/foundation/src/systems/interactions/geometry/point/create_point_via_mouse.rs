use specs::prelude::*;
use geopad_core_lib::{events::*, resources::*, components::symbolics::*};
use crate::{resources::*, events::*};

pub struct CreatePointViaMouse {
  tool_change_event_reader: Option<ToolChangeEventReader>,
  mouse_event_reader: Option<MouseEventReader>,
}

impl Default for CreatePointViaMouse {
  fn default() -> Self {
    Self {
      tool_change_event_reader: None,
      mouse_event_reader: None,
    }
  }
}

impl<'a> System<'a> for CreatePointViaMouse {
  type SystemData = (
    Read<'a, MaybeSnapPoint>,
    Read<'a, Viewport>,
    Read<'a, ToolChangeEventChannel>,
    Write<'a, MouseEventChannel>,
    Write<'a, CommandEventChannel>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.mouse_event_reader = Some(world.fetch_mut::<MouseEventChannel>().register_reader());
  }

  fn run(&mut self, (
    maybe_snap_point,
    viewport,
    tool_change_event_channel,
    mut mouse_event_channel,
    mut command_event_channel,
  ): Self::SystemData) {
    if let Some(reader) = &mut self.tool_change_event_reader {
      for ToolChangeEvent(tool) in tool_change_event_channel.read(reader) {
        if tool.need_snap_point() {
          if self.mouse_event_reader.is_none() {
            self.mouse_event_reader = Some(mouse_event_channel.register_reader());
          }
        } else if let Some(reader) = &mut self.mouse_event_reader {
          std::mem::drop(reader);
          self.mouse_event_reader = None;
        }
      }
    }

    if let Some(reader) = &mut self.mouse_event_reader {
      for event in mouse_event_channel.read(reader) {
        match event {
          MouseEvent::MouseDown(_) => {
            if let Some(SnapPoint { position, symbol }) = maybe_snap_point.get() {
              let maybe_sym_point = match symbol {
                SnapPointType::NotSnapped => Some(SymbolicPoint::Free(position.to_virtual(&*viewport))),
                SnapPointType::SnapOnLine(l_ent, t) => Some(SymbolicPoint::OnLine(l_ent, t.into())),
                SnapPointType::SnapOnLineLineIntersection(l1_ent, l2_ent) => Some(SymbolicPoint::LineLineIntersect(l1_ent, l2_ent)),
                SnapPointType::SnapOnCircle(c_ent, theta) => Some(SymbolicPoint::OnCircle(c_ent, theta)),
                SnapPointType::SnapOnCircleLineIntersection(c_ent, l_ent, id) => Some(SymbolicPoint::CircleLineIntersect(c_ent, l_ent, id)),
                SnapPointType::SnapOnCircleCircleIntersection(c1_ent, c2_ent, id) => Some(SymbolicPoint::CircleCircleIntersect(c1_ent, c2_ent, id)),
                SnapPointType::SnapOnPoint(_) => None,
              };
              if let Some(sym_point) = maybe_sym_point {
                command_event_channel.single_write(CommandEvent::PointInsert(InsertPointEvent::InsertPoint(sym_point)));
              }
            }
          },
          _ => (),
        }
      }
    }
  }
}