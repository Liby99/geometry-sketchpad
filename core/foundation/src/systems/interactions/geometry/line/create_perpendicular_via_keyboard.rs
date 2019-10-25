use specs::prelude::*;
use geopad_core_lib::{events::*, components::{symbolics::*, markers::*}};
use crate::{resources::*, utilities::*};

#[derive(Default)]
pub struct CreatePerpendicularViaKeyboard;

impl<'a> System<'a> for CreatePerpendicularViaKeyboard {
  type SystemData = (
    Entities<'a>,
    Read<'a, InputState>,
    Write<'a, CommandEventChannel>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    input_state,
    mut command_event_channel,
    sym_points,
    sym_lines,
    selecteds,
  ): Self::SystemData) {
    let cmd = input_state.keyboard.is_command_activated();
    let shift = input_state.keyboard.is_shift_activated();
    let backslash = input_state.keyboard.just_activated(Key::Backslash);
    if cmd && shift && backslash {
      if let Some((line_ent, point_ents)) = check_perp_para_selection(&entities, &sym_points, &sym_lines, &selecteds) {
        for point_ent in point_ents {
          let sym_line = SymbolicLine::Perpendicular(line_ent, point_ent);
          command_event_channel.single_write(CommandEvent::LineInsert(InsertLineEvent::InsertLine(sym_line)));
        }
      }
    }
  }
}