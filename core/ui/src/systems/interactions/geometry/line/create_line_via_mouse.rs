use specs::prelude::*;
use core_lib::{math::*, components::symbolics::*, events::*};
use crate::{events::*, resources::*};

pub struct CreateLineViaMouse {
  tool_change_event_reader: Option<ToolChangeEventReader>,
  active_point_event_reader: Option<ActivePointEventReader>,
}

impl Default for CreateLineViaMouse {
  fn default() -> Self {
    Self {
      tool_change_event_reader: None,
      active_point_event_reader: None,
    }
  }
}

impl<'a> System<'a> for CreateLineViaMouse {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, ToolState>,
    Write<'a, SnapLine>,
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
    tool_state,
    mut snap_line,
    tool_change_event_channel,
    mut active_point_event_reader,
    mut command_event_channel,
  ): Self::SystemData) {

    if let Some(reader) = &mut self.tool_change_event_reader {
      for ToolChangeEvent(tool) in tool_change_event_channel.read(reader) {
        match tool {
          Tool::Line(_) => {
            if self.active_point_event_reader.is_none() {
              self.active_point_event_reader = Some(active_point_event_reader.register_reader())
            }
          },
          _ => {
            if let Some(reader) = &mut self.active_point_event_reader {
              std::mem::drop(reader);
              self.active_point_event_reader = None;
              snap_line.maybe_first_point = None;
            }
          }
        }
      }
    }

    if input_state.keyboard.just_activated(Key::Escape) {
      if snap_line.maybe_first_point.is_some() {
        snap_line.maybe_first_point = None;
      }
    }

    if let Some(reader) = &mut self.active_point_event_reader {
      for ActivePointEvent(ent) in active_point_event_reader.read(reader) {
        let curr_ent = *ent;
        if let Some(first_point_ent) = snap_line.maybe_first_point {
          if first_point_ent != curr_ent {
            if let Tool::Line(line_type) = tool_state.get() {
              let sym_line = match line_type {
                LineType::Straight => SymbolicLine::Straight(first_point_ent, curr_ent),
                LineType::Ray => SymbolicLine::Ray(first_point_ent, curr_ent),
                LineType::Segment => SymbolicLine::Segment(first_point_ent, curr_ent),
              };
              command_event_channel.single_write(CommandEvent::LineInsert(InsertLineEvent::InsertLine(sym_line)));
              snap_line.maybe_first_point = None;
            }
          }
        } else {
          snap_line.maybe_first_point = Some(curr_ent);
        }
      }
    }
  }
}