use specs::prelude::*;
use crate::{
  resources::events::{GeometryAction, GeometryActionChannel, GeometryActionReader},
  components::{SymbolicLine, SymbolicPoint, Selected},
};

pub struct DrawPerpOnSelected {
  geometry_action_reader: Option<GeometryActionReader>,
}

impl Default for DrawPerpOnSelected {
  fn default() -> Self {
    Self { geometry_action_reader: None }
  }
}

impl<'a> System<'a> for DrawPerpOnSelected {
  type SystemData = (
    Entities<'a>,
    Write<'a, GeometryActionChannel>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_action_reader = Some(world.fetch_mut::<GeometryActionChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    mut geometry_action_channel,
    sym_points,
    sym_lines,
    selected,
  ): Self::SystemData) {
    let mut to_insert = vec![];

    // Find the symbolic lines to insert
    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_action_channel.read(reader_id) {
        match event {
          GeometryAction::DrawPerpendicularOnSelected => {

            // Main functionality starts
            let mut maybe_line_ent = None;
            for (entity, _, _) in (&entities, &sym_lines, &selected).join() {
              if maybe_line_ent.is_none() {
                maybe_line_ent = Some(entity);
              } else {
                return; // Early terminate since we don't accept more than one line being selected
              }
            }

            if let Some(line_ent) = maybe_line_ent {
              for (p_ent, _, _) in (&entities, &sym_points, &selected).join() {
                let sym_line = SymbolicLine::Perpendicular(line_ent, p_ent);
                to_insert.push(sym_line);
              }
            }

            break;
          },
          _ => (),
        }
      }
    }

    // Submit the events
    for sym_line in to_insert {
      geometry_action_channel.single_write(GeometryAction::InsertLine(sym_line));
    }
  }
}