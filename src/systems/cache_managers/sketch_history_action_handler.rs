use std::collections::{HashMap, HashSet};
use specs::prelude::*;
use crate::{
  resources::{
    events::{
      HistoryAction, HistoryActionReader, HistoryActionChannel,
      SketchEvent, SketchEventChannel, SketchGeometry,
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
                SketchHistoryEvent::InsertMany(insertions) => remove(&mut sketch_event_channel, insertions),
                SketchHistoryEvent::RemoveMany(removals) => insert(&mut sketch_event_channel, removals),
                SketchHistoryEvent::HideMany(entities) => unhide(&mut sketch_event_channel, entities),
                SketchHistoryEvent::UnhideMany(entities) => hide(&mut sketch_event_channel, entities),
              }
            }
          },
          HistoryAction::Redo => {
            if let Some(history_event) = sketch_history.redo() {
              match history_event {
                SketchHistoryEvent::InsertMany(insertions) => insert(&mut sketch_event_channel, insertions),
                SketchHistoryEvent::RemoveMany(removals) => remove(&mut sketch_event_channel, removals),
                SketchHistoryEvent::HideMany(entities) => hide(&mut sketch_event_channel, entities),
                SketchHistoryEvent::UnhideMany(entities) => unhide(&mut sketch_event_channel, entities),
              }
            }
          },
          // _ => (),
        }
      }
    }
  }
}

fn remove(sketch_event_channel: &mut SketchEventChannel, entities: &HashMap<Entity, SketchGeometry>) {
  for (entity, geometry) in entities {
    sketch_event_channel.single_write(SketchEvent::remove_by_history(*entity, *geometry));
  }
}

fn insert(sketch_event_channel: &mut SketchEventChannel, entities: &HashMap<Entity, SketchGeometry>) {
  for (entity, geometry) in entities {
    sketch_event_channel.single_write(SketchEvent::insert_by_history(*entity, *geometry));
  }
}

fn hide(sketch_event_channel: &mut SketchEventChannel, entities: &HashSet<Entity>) {
  for entity in entities {
    sketch_event_channel.single_write(SketchEvent::hide_by_history(*entity));
  }
}

fn unhide(sketch_event_channel: &mut SketchEventChannel, entities: &HashSet<Entity>) {
  for entity in entities {
    sketch_event_channel.single_write(SketchEvent::unhide_by_history(*entity));
  }
}