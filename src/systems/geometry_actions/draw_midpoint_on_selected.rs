use specs::prelude::*;
use crate::{
  resources::events::{GeometryAction, GeometryActionChannel, GeometryActionReader},
  components::{SymbolicPoint, Selected},
};

pub struct DrawMidpointOnSelected {
  geometry_action_reader: Option<GeometryActionReader>,
}

impl Default for DrawMidpointOnSelected {
  fn default() -> Self {
    Self { geometry_action_reader: None }
  }
}

impl<'a> System<'a> for DrawMidpointOnSelected {
  type SystemData = (
    Entities<'a>,
    Write<'a, GeometryActionChannel>,
    ReadStorage<'a, SymbolicPoint>,
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
    selected,
  ): Self::SystemData) {
    let mut to_insert = None;

    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_action_channel.read(reader_id) {
        match event {
          GeometryAction::DrawMidpointOnSelected => {

            let mut abort = false;
            let mut p1 = None;
            let mut p2 = None;
            for (entity, _, _) in (&entities, &sym_points, &selected).join() {
              if p1.is_none() {
                p1 = Some(entity);
              } else if p2.is_none() {
                p2 = Some(entity);
              } else {
                abort = true;
              }
            }

            if !abort {
              if let Some(entity_1) = p1 {
                if let Some(entity_2) = p2 {
                  to_insert = Some(SymbolicPoint::MidPoint(entity_1, entity_2));
                }
              }
            }

            break;
          },
          _ => (),
        }
      }
    }

    if let Some(sym_point) = to_insert {
      geometry_action_channel.single_write(GeometryAction::InsertPoint(sym_point));
    }
  }
}