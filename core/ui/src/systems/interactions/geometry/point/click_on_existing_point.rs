use crate::{events::*, resources::*};
use specs::prelude::*;

pub struct ClickOnExistingPoint {
  mouse_event_reader: Option<MouseEventReader>,
}

impl Default for ClickOnExistingPoint {
  fn default() -> Self {
    Self {
      mouse_event_reader: None,
    }
  }
}

impl<'a> System<'a> for ClickOnExistingPoint {
  type SystemData = (
    Read<'a, MaybeSnapPoint>,
    Read<'a, MouseEventChannel>,
    Write<'a, ActivePointEventChannel>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.mouse_event_reader = Some(world.fetch_mut::<MouseEventChannel>().register_reader());
  }

  fn run(&mut self, (maybe_snap_point, mouse_event_channel, mut active_point_event_channel): Self::SystemData) {
    if let Some(reader) = &mut self.mouse_event_reader {
      for event in mouse_event_channel.read(reader) {
        match event {
          MouseEvent::MouseDown(_) => {
            if let Some(SnapPoint { symbol, .. }) = maybe_snap_point.get() {
              match symbol {
                SnapPointType::SnapOnPoint(p_ent) => active_point_event_channel.single_write(ActivePointEvent(p_ent)),
                _ => (),
              }
            }
          }
          _ => (),
        }
      }
    }
  }
}
