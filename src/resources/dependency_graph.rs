use std::collections::{HashMap, HashSet};
use specs::prelude::*;

pub struct DependencyGraph(HashMap<Entity, HashSet<Entity>>);

impl Default for DependencyGraph {
  fn default() -> Self {
    Self(HashMap::new())
  }
}

impl DependencyGraph {
  pub fn add(&mut self, parent: &Entity, child: &Entity) {
    self.0.entry(*parent).or_insert(HashSet::new()).insert(*child);
  }

  pub fn get(&self, parent: &Entity) -> Option<&HashSet<Entity>> {
    self.0.get(parent)
  }

  pub fn remove(&mut self, parent: &Entity) {
    self.0.remove(parent);
  }
}