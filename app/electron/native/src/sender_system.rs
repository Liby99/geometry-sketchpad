use specs::prelude::*;
use core_lib::{
  components::{screen_shapes::*, styles::*},
  events::*,
};
use super::output::RenderUpdateEvent;

pub struct SenderSystem {
  pub sender: std::sync::mpsc::Sender<RenderUpdateEvent>,
  scrn_point_update_reader: Option<ReaderId<ComponentEvent>>,
  marker_event_reader: Option<MarkerEventReader>,
}

impl SenderSystem {
  pub fn new(sender: std::sync::mpsc::Sender<RenderUpdateEvent>) -> Self {
    Self {
      sender,
      scrn_point_update_reader: None,
      marker_event_reader: None,
    }
  }
}

impl<'a> System<'a> for SenderSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, MarkerEventChannel>,
    ReadStorage<'a, ScreenPoint>,
    ReadStorage<'a, PointStyle>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.scrn_point_update_reader = Some(WriteStorage::<ScreenPoint>::fetch(&world).register_reader());
    self.marker_event_reader = Some(world.fetch_mut::<MarkerEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    marker_event_channel,
    scrn_points,
    point_styles,
  ): Self::SystemData) {

    // First deal with geometry update
    let mut dirty : BitSet = BitSet::new();
    let mut to_remove : BitSet = BitSet::new();
    if let Some(reader) = &mut self.scrn_point_update_reader {
      for event in scrn_points.channel().read(reader) {
        match event {
          ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
            dirty.add(*id);
          },
          ComponentEvent::Removed(id) => {
            to_remove.add(*id);
          },
        }
      }
    }

    for (ent, scrn_point, point_style, _) in (&entities, &scrn_points, &point_styles, &dirty).join() {
      if let Err(err) = self.sender.send(RenderUpdateEvent::UpdatedPoint(ent, *scrn_point, *point_style)) { panic!(err) }
    }

    // Then deal with select update
    if let Some(reader) = &mut self.marker_event_reader {
      for event in marker_event_channel.read(reader) {
        match event {
          MarkerEvent::Deselect(ent) => {
            if let Err(err) = self.sender.send(RenderUpdateEvent::DeselectedEntity(*ent)) { panic!(err) }
          },
          MarkerEvent::Select(ent) => {
            if let Err(err) = self.sender.send(RenderUpdateEvent::SelectedEntity(*ent)) { panic!(err) }
          },
          _ => (),
        }
      }
    }
  }
}