use crate::events::HistoryEvent;

pub struct History {
  history: Vec<HistoryEvent>,
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
  pub fn undo(&mut self) -> Option<&HistoryEvent> {
    if self.cursor > 0 {
      self.cursor -= 1;
      Some(&self.history[self.cursor])
    } else {
      None
    }
  }

  pub fn redo(&mut self) -> Option<&HistoryEvent> {
    if self.cursor < self.head {
      self.cursor += 1;
      Some(&self.history[self.cursor - 1])
    } else {
      None
    }
  }

  pub fn push(&mut self, event: HistoryEvent) {
    if self.cursor < self.history.len() {
      self.history[self.cursor] = event;
    } else {
      self.history.push(event);
    }
    self.cursor += 1;
    self.head = self.cursor;
  }
}