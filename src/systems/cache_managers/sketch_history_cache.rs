use std::collections::{HashMap, HashSet};
use specs::prelude::*;
use crate::{
  resources::{
    events::{SketchEvent, SketchEventChannel, SketchEventReader, SketchGeometry},
    SketchHistory, SketchHistoryEvent,
  }
};

enum Event {
  None,
  Insert(HashMap<Entity, SketchGeometry>),
  Remove(HashMap<Entity, SketchGeometry>),
  Hide(HashSet<Entity>),
  Unhide(HashSet<Entity>),
}

pub struct SketchHistoryCache {
  sketch_event_reader: Option<SketchEventReader>,
}

impl Default for SketchHistoryCache {
  fn default() -> Self {
    Self { sketch_event_reader: None }
  }
}

impl<'a> System<'a> for SketchHistoryCache {
  type SystemData = (
    Read<'a, SketchEventChannel>,
    Write<'a, SketchHistory>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.sketch_event_reader = Some(world.fetch_mut::<SketchEventChannel>().register_reader());
  }

  fn run(&mut self, (
    sketch_event_channel,
    mut sketch_history,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.sketch_event_reader {

      let mut curr_event = Event::None;

      for event in sketch_event_channel.read(reader_id) {
        match event {
          SketchEvent::Insert(entity, geom, false) => {
            if let Event::Insert(insertions) = &mut curr_event {
              insertions.insert(*entity, *geom);
            } else {
              push_event(curr_event, &mut sketch_history);
              let mut insertions = HashMap::new();
              insertions.insert(*entity, *geom);
              curr_event = Event::Insert(insertions);
            }
          },
          SketchEvent::Remove(entity, geom, false) => {
            if let Event::Remove(removals) = &mut curr_event {
              removals.insert(*entity, *geom);
            } else {
              push_event(curr_event, &mut sketch_history);
              let mut removals = HashMap::new();
              removals.insert(*entity, *geom);
              curr_event = Event::Remove(removals);
            }
          },
          SketchEvent::Hide(entity, false) => {
            if let Event::Hide(entities) = &mut curr_event {
              entities.insert(*entity);
            } else {
              push_event(curr_event, &mut sketch_history);
              let mut entities = HashSet::new();
              entities.insert(*entity);
              curr_event = Event::Hide(entities);
            }
          },
          SketchEvent::Unhide(entity, false) => {
            if let Event::Unhide(entities) = &mut curr_event {
              entities.insert(*entity);
            } else {
              push_event(curr_event, &mut sketch_history);
              let mut entities = HashSet::new();
              entities.insert(*entity);
              curr_event = Event::Unhide(entities);
            }
          },
          _ => (),
        }
      }

      push_event(curr_event, &mut sketch_history);
    }
  }
}

fn push_event(event: Event, history: &mut SketchHistory) {
  match event {
    Event::None => (),
    Event::Insert(insertions) => history.push(SketchHistoryEvent::InsertMany(insertions)),
    Event::Remove(removals) => history.push(SketchHistoryEvent::RemoveMany(removals)),
    Event::Hide(entities) => history.push(SketchHistoryEvent::HideMany(entities)),
    Event::Unhide(entities) => history.push(SketchHistoryEvent::UnhideMany(entities)),
  }
}