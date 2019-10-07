use specs::prelude::*;
use crate::{
  resources::{
    events::{
      HistoryAction, HistoryActionReader, HistoryActionChannel,
      SketchEvent, SketchEventChannel,
    },
    SketchHistory, SketchHistoryEvent,
  },
};

pub struct SketchHistoryActionHandler {
  history_action_reader: Option<HistoryActionReader>,
}

impl Default for SketchHistoryActionHandler {
  fn default() -> Self {
    Self { history_action_reader: None }
  }
}

impl<'a> System<'a> for SketchHistoryActionHandler {
  type SystemData = (
    Read<'a, HistoryActionChannel>,
    Write<'a, SketchHistory>,
    Write<'a, SketchEventChannel>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.history_action_reader = Some(world.fetch_mut::<HistoryActionChannel>().register_reader());
  }

  fn run(&mut self, (
    history_action_channel,
    mut sketch_history,
    mut sketch_event_channel,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.history_action_reader {
      for event in history_action_channel.read(reader_id) {
        match event {
          HistoryAction::Undo => {
            if let Some(history_event) = sketch_history.undo() {
              match history_event {
                SketchHistoryEvent::InsertMany(insertions) => {
                  for (inserted_entity, inserted_geometry) in insertions {
                    sketch_event_channel.single_write(SketchEvent::remove_by_history(*inserted_entity, *inserted_geometry));
                  }
                },
                SketchHistoryEvent::RemoveMany(removals) => {
                  for (removed_entity, removed_geometry) in removals {
                    sketch_event_channel.single_write(SketchEvent::insert_by_history(*removed_entity, *removed_geometry));
                  }
                },
              }
            }
          },
          HistoryAction::Redo => {
            if let Some(history_event) = sketch_history.redo() {
              match history_event {
                SketchHistoryEvent::InsertMany(insertions) => {
                  for (inserted_entity, inserted_geometry) in insertions {
                    sketch_event_channel.single_write(SketchEvent::insert_by_history(*inserted_entity, *inserted_geometry));
                  }
                },
                SketchHistoryEvent::RemoveMany(removals) => {
                  for (removed_entity, removed_geometry) in removals {
                    sketch_event_channel.single_write(SketchEvent::remove_by_history(*removed_entity, *removed_geometry));
                  }
                },
              }
            }
          },
          // _ => (),
        }
      }
    }
  }
}