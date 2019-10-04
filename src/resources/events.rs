use specs::prelude::*;
use crate::util::Vector2;

pub enum Event {
  Viewport(ViewportEvent),
  Geometry(GeometryEvent),
}

pub enum ViewportEvent {
  // Scale(f32), // Delta, Maybe add later
  Move(Vector2), // Delta
  Resize(Vector2), // New size
}

pub enum GeometryEvent {
  Inserted(Geometry, Entity),
}

pub enum Geometry {
  Point,
}

pub struct Events(Vec<Event>);

impl Default for Events {
  fn default() -> Self {
    Events(Vec::new())
  }
}

impl Events {
  pub fn clear(&mut self) {
    self.0.clear();
  }

  pub fn push(&mut self, event: Event) {
    self.0.push(event);
  }

  pub fn has_viewport_event(&self) -> bool {
    for event in self.0.iter() {
      if let Event::Viewport(_) = event {
        return true;
      }
    }
    false
  }
}

impl IntoIterator for Events {
  type Item = Event;
  type IntoIter = ::std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}