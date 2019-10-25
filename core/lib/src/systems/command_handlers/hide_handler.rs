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
    WriteStorage<'a, Selected>,
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
    mut selecteds,
    mut hiddens
  ): Self::SystemData) {
    if let Some(reader) = &mut self.command_event_reader {
      for event in command_event_channel.read(reader) {
        match event {
          CommandEvent::Hide(hide_event) => match hide_event {
            HideEvent::Hide(ent) => {
              if let Err(err) = hiddens.insert(*ent, Hidden) { panic!(err) }
              selecteds.remove(*ent);
              marker_event_channel.single_write(MarkerEvent::hide(*ent));
            },
            HideEvent::HideByHistory(ent) => {
              if let Err(err) = hiddens.insert(*ent, Hidden) { panic!(err) }
              selecteds.remove(*ent);
              marker_event_channel.single_write(MarkerEvent::hide_by_history(*ent));
            },
            HideEvent::Unhide(ent) => {
              hiddens.remove(*ent);
              if let Err(err) = selecteds.insert(*ent, Selected) { panic!(err) }
              marker_event_channel.single_write(MarkerEvent::unhide(*ent));
            },
            HideEvent::UnhideByHistory(ent) => {
              hiddens.remove(*ent);
              if let Err(err) = selecteds.insert(*ent, Selected) { panic!(err) }
              marker_event_channel.single_write(MarkerEvent::unhide_by_history(*ent));
            },
            HideEvent::HideSelected => {
              let mut to_hide = Vec::new();
              for (ent, _) in (&entities, &selecteds).join() { to_hide.push(ent) }
              for ent in to_hide {
                selecteds.remove(ent);
                if let Err(err) = hiddens.insert(ent, Hidden) { panic!(err) }
                marker_event_channel.single_write(MarkerEvent::hide(ent));
              }
            },
            HideEvent::UnhideAll => {
              for (ent, _) in (&entities, &hiddens).join() {
                if let Err(err) = selecteds.insert(ent, Selected) { panic!(err) }
                marker_event_channel.single_write(MarkerEvent::unhide(ent));
              }
              hiddens.clear();
            },
          },
          _ => (),
        }
      }
    }
  }
}