use specs::prelude::*;
use shrev::*;

pub enum MarkerEvent {
  Select(Entity),
  Deselect(Entity),
  Hide(Entity),
  Unhide(Entity),
}

pub type MarkerEventChannel = EventChannel<MarkerEvent>;

pub type MarkerEventReader = ReaderId<MarkerEvent>;