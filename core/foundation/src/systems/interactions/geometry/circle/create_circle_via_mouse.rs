use specs::prelude::*;
use geopad_core_lib::{components::symbolics::*, events::*};
use crate::{events::*, resources::*};

pub struct CreateCircleViaMouse {
  tool_change_event_reader: Option<ToolChangeEventReader>,
  active_point_event_reader: Option<ActivePointEventReader>,
}

impl Default for CreateCircleViaMouse {
  fn default() -> Self {
    Self {
      tool_change_event_reader: None,
      active_point_event_reader: None,
    }
  }
}

impl<'a> System<'a> for CreateCircleViaMouse {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, SnapCircle>,
    Read<'a, ToolChangeEventChannel>,
    Write<'a, ActivePointEventChannel>,
    Write<'a, CommandEventChannel>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_event_reader = Some(world.fetch_mut::<ToolChangeEventChannel>().register_reader());
  }

  fn run(&mut self, (
    input_state,
    mut snap_circle,
    tool_change_event_channel,
    mut active_point_event_reader,
    mut command_event_channel,
  ): Self::SystemData) {

    if let Some(reader) = &mut self.tool_change_event_reader {
      for ToolChangeEvent(tool) in tool_change_event_channel.read(reader) {
        match tool {
          Tool::Circle => {
            if self.active_point_event_reader.is_none() {
              self.active_point_event_reader = Some(active_point_event_reader.register_reader())
            }
          },
          _ => {
            if let Some(reader) = &mut self.active_point_event_reader {
              std::mem::drop(reader);
              self.active_point_event_reader = None;
              snap_circle.maybe_first_point = None;
            }
          }
        }
      }
    }

    if input_state.keyboard.just_activated(Key::Escape) {
      if snap_circle.maybe_first_point.is_some() {
        snap_circle.maybe_first_point = None;
      }
    }

    if let Some(reader) = &mut self.active_point_event_reader {
      for ActivePointEvent(ent) in active_point_event_reader.read(reader) {
        let curr_ent = *ent;
        if let Some(first_point_ent) = snap_circle.maybe_first_point {
          if first_point_ent != curr_ent {
            let sym_circle = SymbolicCircle::CenterRadius(first_point_ent, curr_ent);
            command_event_channel.single_write(CommandEvent::CircleInsert(InsertCircleEvent::InsertCircle(sym_circle)));
            snap_circle.maybe_first_point = None;
          }
        } else {
          snap_circle.maybe_first_point = Some(curr_ent);
        }
      }
    }
  }
}