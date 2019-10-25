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

impl MarkerEvent {
  pub fn hide(ent: Entity) -> Self {
    MarkerEvent::Hide(ent, false)
  }

  pub fn hide_by_history(ent: Entity) -> Self {
    MarkerEvent::Hide(ent, true)
  }

  pub fn unhide(ent: Entity) -> Self {
    MarkerEvent::Unhide(ent, false)
  }

  pub fn unhide_by_history(ent: Entity) -> Self {
    MarkerEvent::Unhide(ent, true)
  }
}