use std::collections::{HashMap, HashSet};
use specs::prelude::*;
use crate::utilities::Geometry;

pub enum HistoryEvent {
  RemoveMany(HashMap<Entity, Geometry>),
  InsertMany(HashMap<Entity, Geometry>),
  Update(Entity, Geometry, Geometry), // Entity, old, new
  HideMany(HashSet<Entity>),
  UnhideMany(HashSet<Entity>),
}