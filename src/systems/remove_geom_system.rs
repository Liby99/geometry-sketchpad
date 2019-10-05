use std::collections::HashSet;
use specs::prelude::*;
use crate::{
  util::Key,
  resources::{DependencyGraph, InputState},
  components::*,
};

pub struct RemoveGeomSystem;

impl<'a> System<'a> for RemoveGeomSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, InputState>,
    Write<'a, DependencyGraph>,
    WriteStorage<'a, Point>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, Line>,
    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    input_state,
    mut dep_graph,
    mut points,
    mut sym_points,
    mut point_styles,
    mut lines,
    mut sym_lines,
    mut line_styles,
    mut selected,
  ): Self::SystemData) {
    if input_state.keyboard.just_activated(Key::Backspace) || input_state.keyboard.just_activated(Key::Delete) {
      let mut to_remove = HashSet::new();
      let mut stack = vec![];

      // Add all the selected stuff to stack
      for (entity, _) in (&entities, &selected).join() {
        stack.push(entity);
      }

      // Explore the stack in dependency graph and put all of the children to `to_remove`
      while let Some(entity) = stack.pop() {
        to_remove.insert(entity);
        for child in dep_graph.get(&entity) {
          stack.push(*child);
        }
        dep_graph.remove(&entity);
      }

      // Remove everything
      for entity in to_remove {
        points.remove(entity);
        sym_points.remove(entity);
        point_styles.remove(entity);
        lines.remove(entity);
        sym_lines.remove(entity);
        line_styles.remove(entity);
        selected.remove(entity);
      }
    }
  }
}