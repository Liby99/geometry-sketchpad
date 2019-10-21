use std::collections::{HashMap, HashSet};
use specs::prelude::*;
use crate::{events::*, resources::*, utilities::*};

enum Mod {
  None,
  Insert(HashMap<Entity, Geometry>),
  Remove(HashMap<Entity, Geometry>),
  Hide(HashSet<Entity>),
  Unhide(HashSet<Entity>),
}

pub struct HistoryManager {
  geometry_event_reader: Option<GeometryEventReader>,
  marker_event_reader: Option<MarkerEventReader>,
}

impl Default for HistoryManager {
  fn default() -> Self {
    Self {
      geometry_event_reader: None,
      marker_event_reader: None,
    }
  }
}

impl<'a> System<'a> for HistoryManager {
  type SystemData = (
    Read<'a, GeometryEventChannel>,
    Read<'a, MarkerEventChannel>,
    Write<'a, History>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_event_reader = Some(world.fetch_mut::<GeometryEventChannel>().register_reader());
  }

  fn run(&mut self, (
    geometry_event_channel,
    marker_event_channel,
    mut history,
  ): Self::SystemData) {

    // First do geometry events
    if let Some(reader_id) = &mut self.geometry_event_reader {
      let mut curr_event = Mod::None;
      for event in geometry_event_channel.read(reader_id) {
        match event {
          GeometryEvent::Inserted(entity, geom, false) => {
            if let Mod::Insert(insertions) = &mut curr_event {
              insertions.insert(*entity, *geom);
            } else {
              push_event(curr_event, &mut history);
              let mut insertions = HashMap::new();
              insertions.insert(*entity, *geom);
              curr_event = Mod::Insert(insertions);
            }
          },
          GeometryEvent::Removed(entity, geom, false) => {
            if let Mod::Remove(removals) = &mut curr_event {
              removals.insert(*entity, *geom);
            } else {
              push_event(curr_event, &mut history);
              let mut removals = HashMap::new();
              removals.insert(*entity, *geom);
              curr_event = Mod::Remove(removals);
            }
          },
          _ => (),
        }
      }
      push_event(curr_event, &mut history);
    }

    // Then do marker events
    if let Some(reader) = &mut self.marker_event_reader {
      let mut curr_event = Mod::None;
      for event in marker_event_channel.read(reader) {
        match event {
          MarkerEvent::Hide(entity, false) => {
            if let Mod::Hide(entities) = &mut curr_event {
              entities.insert(*entity);
            } else {
              push_event(curr_event, &mut history);
              let mut entities = HashSet::new();
              entities.insert(*entity);
              curr_event = Mod::Hide(entities);
            }
          },
          MarkerEvent::Unhide(entity, false) => {
            if let Mod::Unhide(entities) = &mut curr_event {
              entities.insert(*entity);
            } else {
              push_event(curr_event, &mut history);
              let mut entities = HashSet::new();
              entities.insert(*entity);
              curr_event = Mod::Unhide(entities);
            }
          },
          _ => (),
        }
      }
      push_event(curr_event, &mut history);
    }
  }
}

fn push_event(event: Mod, history: &mut History) {
  match event {
    Mod::None => (),
    Mod::Insert(insertions) => history.push(Modification::InsertMany(insertions)),
    Mod::Remove(removals) => history.push(Modification::RemoveMany(removals)),
    Mod::Hide(entities) => history.push(Modification::HideMany(entities)),
    Mod::Unhide(entities) => history.push(Modification::UnhideMany(entities)),
  }
}