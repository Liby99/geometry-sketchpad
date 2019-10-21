use specs::prelude::*;
use shrev::*;

pub enum MarkerEvent {
  Select(Entity),
  Deselect(Entity),
  Hide(Entity, bool), // bool: Is done by history
  Unhide(Entity, bool), // bool: Is done by history
}

pub type MarkerEventChannel = EventChannel<MarkerEvent>;

pub type MarkerEventReader = ReaderId<MarkerEvent>;