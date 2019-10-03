use std::collections::{HashMap, HashSet};
use specs::Entity;

pub struct DescendantMap(HashMap<Entity, HashSet<Entity>>);

impl Default for DescendantMap {
  fn default() -> Self {
    Self(HashMap::new())
  }
}

impl DescendantMap {
  pub fn add_descendant(&mut self, parent: Entity, child: Entity) {
    self.0.entry(parent).or_insert(HashSet::new()).insert(child);
  }

  pub fn get_descendants(&mut self, parent: Entity) -> std::collections::hash_set::Iter<Entity> {
    self.0.entry(parent).or_insert(HashSet::new()).iter()
  }

  pub fn has_descendant(&self, parent: Entity, child: Entity) -> bool {
    if let Some(set) = self.0.get(&parent) { set.contains(&child) } else { false }
  }
}