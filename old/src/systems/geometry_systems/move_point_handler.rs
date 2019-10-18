use specs::prelude::*;
use crate::{
  resources::events::{SketchEvent, SketchEventChannel, SketchEventReader, MovePoint},
  components::SymbolicPoint,
};

pub struct MovePointHandler {
  sketch_event_reader: Option<SketchEventReader>,
}

impl Default for MovePointHandler {
  fn default() -> Self {
    Self { sketch_event_reader: None }
  }
}

impl<'a> System<'a> for MovePointHandler {
  type SystemData = (
    Read<'a, SketchEventChannel>,
    WriteStorage<'a, SymbolicPoint>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.sketch_event_reader = Some(world.fetch_mut::<SketchEventChannel>().register_reader());
  }

  fn run(&mut self, (sketch_event_channel, mut sym_points): Self::SystemData) {
    if let Some(reader_id) = &mut self.sketch_event_reader {
      for event in sketch_event_channel.read(reader_id) {
        match event {
          SketchEvent::MovePoint(entity, move_point) => {
            match move_point {
              MovePoint::Free(_, new_position) => {
                if let Err(err) = sym_points.insert(*entity, SymbolicPoint::Free(*new_position)) { panic!(err) }
              },
              MovePoint::OnLine(line_entity, _, new_t) => {
                if let Err(err) = sym_points.insert(*entity, SymbolicPoint::OnLine(*line_entity, *new_t)) { panic!(err) }
              },
              MovePoint::OnCircle(circ_entity, _, new_theta) => {
                if let Err(err) = sym_points.insert(*entity, SymbolicPoint::OnCircle(*circ_entity, *new_theta)) { panic!(err) }
              },
            }
          },
          _ => (),
        }
      }
    }
  }
}