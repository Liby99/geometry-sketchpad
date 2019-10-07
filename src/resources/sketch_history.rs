use std::collections::HashMap;
use specs::prelude::*;
use crate::{
  components::{SymbolicLine, SymbolicPoint, PointStyle, LineStyle},
};

pub struct SketchHistory {
  history: Vec<UserSketchEvent>,
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

pub enum UserSketchEvent {
  RemoveOne(Entity, SketchGeometry),
  RemoveMany(HashMap<Entity, SketchGeometry>),
  InsertOne(Entity, SketchGeometry),
  InsertMany(HashMap<Entity, SketchGeometry>),
  Update(Entity, SketchGeometry, SketchGeometry), // Entity, old, new
}

#[derive(Debug, Clone, Copy)]
pub enum SketchGeometry {
  Point(SymbolicPoint, PointStyle),
  Line(SymbolicLine, LineStyle),
}

impl SketchHistory {
  pub fn undo(&mut self) -> Option<&UserSketchEvent> {
    if self.cursor > 0 {
      self.cursor -= 1;
      Some(&self.history[self.cursor])
    } else {
      None
    }
  }

  pub fn redo(&mut self) -> Option<&UserSketchEvent> {
    if self.cursor < self.head {
      self.cursor += 1;
      Some(&self.history[self.cursor - 1])
    } else {
      None
    }
  }

  pub fn push(&mut self, event: UserSketchEvent) {
    if self.cursor < self.head {
      self.history[self.cursor] = event;
    } else {
      self.history.push(event);
    }
    self.cursor += 1;
    self.head = self.cursor;
  }
}