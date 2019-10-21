use shrev::*;

pub enum HistoryEvent {
  Redo,
  Undo,
  ClearCache,
}

pub type HistoryEventChannel = EventChannel<HistoryEvent>;

pub type HistoryEventReader = ReaderId<HistoryEvent>;