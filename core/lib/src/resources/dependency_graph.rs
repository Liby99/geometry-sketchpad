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

  pub fn remove(&mut self, parent: &Entity) {
    self.0.remove(parent);
  }

  pub fn remove_dependent(&mut self, parent: &Entity, child: &Entity) {
    self.0.entry(*parent).and_modify(|set| { set.remove(child); });
  }

  pub fn get_direct_dependents(&self, parent: &Entity) -> Option<&HashSet<Entity>> {
    self.0.get(parent)
  }

  /// Get all the dependents of the parent, including parent itself
  pub fn get_all_dependents(&self, parent: &Entity) -> HashSet<Entity> {
    let mut result : HashSet<Entity> = HashSet::new();
    let mut stack : Vec<Entity> = vec![*parent];

    while let Some(dependent) = stack.pop() {
      result.insert(dependent);
      if let Some(children) = self.0.get(&dependent) {
        for child in children {
          stack.push(*child);
        }
      }
    }

    result
  }
}