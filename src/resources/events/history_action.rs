use shrev::{EventChannel, ReaderId};

#[derive(Debug, Copy, Clone)]
pub enum HistoryAction {
  Undo,
  Redo,
}

pub type HistoryActionChannel = EventChannel<HistoryAction>;

pub type HistoryActionReader = ReaderId<HistoryAction>;