use specs::prelude::*;
use crate::{events::*, components::markers::*};

pub struct HideHandler {
  command_event_reader: Option<CommandEventReader>,
}

impl Default for HideHandler {
  fn default() -> Self {
    Self { command_event_reader: None }
  }
}

impl<'a> System<'a> for HideHandler {
  type SystemData = (
    Entities<'a>,
    Read<'a, CommandEventChannel>,
    Write<'a, MarkerEventChannel>,
    ReadStorage<'a, Selected>,
    WriteStorage<'a, Hidden>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.command_event_reader = Some(world.fetch_mut::<CommandEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    command_event_channel,
    mut marker_event_channel,
    selecteds,
    mut hiddens
  ): Self::SystemData) {
    if let Some(reader) = &mut self.command_event_reader {
      for event in command_event_channel.read(reader) {
        match event {
          CommandEvent::Hide(ent) => {
            if let Err(err) = hiddens.insert(*ent, Hidden) { panic!(err) }
            marker_event_channel.single_write(MarkerEvent::hide(*ent));
          },
          CommandEvent::Unhide(ent) => {
            hiddens.remove(*ent);
            marker_event_channel.single_write(MarkerEvent::unhide(*ent));
          },
          CommandEvent::HideSelected => {
            for (ent, _) in (&entities, &selecteds).join() {
              if let Err(err) = hiddens.insert(ent, Hidden) { panic!(err) }
              marker_event_channel.single_write(MarkerEvent::hide(ent));
            }
          },
          CommandEvent::UnhideAll => {
            for (ent, _) in (&entities, &hiddens).join() {
              marker_event_channel.single_write(MarkerEvent::unhide(ent));
            }
            hiddens.clear();
          },
          _ => (),
        }
      }
    }
  }
}