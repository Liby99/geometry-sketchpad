use specs::prelude::*;
use geopad_core_lib::{events::*, components::{symbolics::*, markers::*}};
use crate::{utilities::*, resources::*};

#[derive(Default)]
pub struct CreateMidpointViaKeyboard;

impl<'a> System<'a> for CreateMidpointViaKeyboard {
  type SystemData = (
    Entities<'a>,
    Read<'a, InputState>,
    Write<'a, CommandEventChannel>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    input_state,
    mut command_event_channel,
    sym_points,
    selecteds,
  ): Self::SystemData) {
    let cmd = input_state.keyboard.is_command_activated();
    let no_shift = !input_state.keyboard.is_shift_activated();
    let m = input_state.keyboard.just_activated(Key::M);
    if cmd && no_shift && m {
      if let Some(sym_point) = create_midpoint_from_selection(&entities, &sym_points, &selecteds) {
        command_event_channel.single_write(CommandEvent::PointInsert(InsertPointEvent::InsertPoint(sym_point)));
      }
    }
  }
}