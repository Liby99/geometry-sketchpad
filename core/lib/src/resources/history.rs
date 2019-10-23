use std::collections::{HashMap, HashSet};
use specs::prelude::*;
use crate::utilities::Geometry;

pub enum Modification {
  RemoveMany(HashMap<Entity, Geometry>),
  InsertMany(HashMap<Entity, Geometry>),
  Update(Entity, Geometry, Geometry), // Entity, old, new
  HideMany(HashSet<Entity>),
  UnhideMany(HashSet<Entity>),
}

pub struct History {
  history: Vec<Modification>,
  cursor: usize,
  head: usize,
}

impl Default for History {
  fn default() -> Self {
    Self {
      history: Vec::new(),
      cursor: 0,
      head: 0,
    }
  }
}

impl History {
  pub fn clear(&mut self) {
    self.history.clear();
    self.cursor = 0;
    self.head = 0;
  }

  pub fn undo(&mut self) -> Option<&Modification> {
    if self.cursor > 0 {
      self.cursor -= 1;
      Some(&self.history[self.cursor])
    } else {
      None
    }
  }

  pub fn redo(&mut self) -> Option<&Modification> {
    if self.cursor < self.head {
      self.cursor += 1;
      Some(&self.history[self.cursor - 1])
    } else {
      None
    }
  }

  pub fn push(&mut self, event: Modification) {
    if self.cursor < self.history.len() {
      self.history[self.cursor] = event;
    } else {
      self.history.push(event);
    }
    self.cursor += 1;
    self.head = self.cursor;
  }
}