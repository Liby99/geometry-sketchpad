use std::collections::HashMap;
use specs::prelude::*;
use crate::events::SketchGeometry;

pub struct SketchHistory {
  history: Vec<SketchHistoryEvent>,
  cursor: usize,
  head: usize,
}

impl Default for SketchHistory {
  fn default() -> Self {
    Self {
      history: Vec::new(),
      cursor: 0,
      head: 0,
    }
  }
}

pub enum SketchHistoryEvent {
  RemoveMany(HashMap<Entity, SketchGeometry>),
  InsertMany(HashMap<Entity, SketchGeometry>),
  // Update(Entity, SketchGeometry, SketchGeometry), // Entity, old, new
}

impl SketchHistory {
  pub fn undo(&mut self) -> Option<&SketchHistoryEvent> {
    if self.cursor > 0 {
      self.cursor -= 1;
      Some(&self.history[self.cursor])
    } else {
      None
    }
  }

  pub fn redo(&mut self) -> Option<&SketchHistoryEvent> {
    if self.cursor < self.head {
      self.cursor += 1;
      Some(&self.history[self.cursor - 1])
    } else {
      None
    }
  }

  pub fn push(&mut self, event: SketchHistoryEvent) {
    if self.cursor < self.history.len() {
      self.history[self.cursor] = event;
    } else {
      self.history.push(event);
    }
    self.cursor += 1;
    self.head = self.cursor;
  }
}