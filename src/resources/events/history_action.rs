use shrev::{EventChannel, ReaderId};

pub enum HistoryAction {
  Undo,
  Redo,
}

pub type HistoryActionChannel = EventChannel<HistoryAction>;

pub type HistoryActionReader = ReaderId<HistoryAction>;