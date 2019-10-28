use specs::prelude::*;
use core_lib::{
  components::{screen_shapes::*, styles::*},
  events::*,
};
use crate::events::*;

pub struct SenderSystem {
  pub sender: std::sync::mpsc::Sender<RenderUpdateEvent>,
  scrn_point_update_reader: Option<ReaderId<ComponentEvent>>,
  point_style_update_reader: Option<ReaderId<ComponentEvent>>,
  marker_event_reader: Option<MarkerEventReader>,
}

impl SenderSystem {
  pub fn new(sender: std::sync::mpsc::Sender<RenderUpdateEvent>) -> Self {
    Self {
      sender,
      scrn_point_update_reader: None,
      point_style_update_reader: None,
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
    self.point_style_update_reader = Some(WriteStorage::<PointStyle>::fetch(&world).register_reader());
    self.marker_event_reader = Some(world.fetch_mut::<MarkerEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    marker_event_channel,
    scrn_points,
    point_styles,
  ): Self::SystemData) {

    // First deal with geometry update
    let mut inserted_points : BitSet = BitSet::new();
    let mut modified_points : BitSet = BitSet::new();
    let mut modified_point_styles : BitSet = BitSet::new();
    let mut removed : BitSet = BitSet::new();

    // Screen point updates
    if let Some(reader) = &mut self.scrn_point_update_reader {
      for event in scrn_points.channel().read(reader) {
        match event {
          ComponentEvent::Inserted(id) => { inserted_points.add(*id); },
          ComponentEvent::Modified(id) => { modified_points.add(*id); },
          ComponentEvent::Removed(id) => { removed.add(*id); },
        }
      }
    }

    if let Some(reader) = &mut self.point_style_update_reader {
      for event in point_styles.channel().read(reader) {
        match event {
          ComponentEvent::Modified(id) => { modified_point_styles.add(*id); },
          _ => (),
        }
      }
    }

    // Do all the insert
    for (ent, scrn_point, point_style, _) in (&entities, &scrn_points, &point_styles, &inserted_points).join() {
      if let Err(err) = self.sender.send(RenderUpdateEvent::InsertedPoint(ent, *scrn_point, *point_style)) { panic!(err) }
    }

    // Do all the modify
    for (ent, scrn_point, _) in (&entities, &scrn_points, &modified_points).join() {
      if let Err(err) = self.sender.send(RenderUpdateEvent::UpdatedPoint(ent, *scrn_point)) { panic!(err) }
    }
    for (ent, point_style, _) in (&entities, &point_styles, &modified_point_styles).join() {
      if let Err(err) = self.sender.send(RenderUpdateEvent::UpdatedPointStyle(ent, *point_style)) { panic!(err) }
    }

    // Do all the removals
    for (ent, _) in (&entities, &removed).join() {
      if let Err(err) = self.sender.send(RenderUpdateEvent::RemovedEntity(ent)) { panic!(err) }
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