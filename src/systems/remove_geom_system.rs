use std::collections::HashSet;
use specs::prelude::*;
use shrev::EventChannel;
use crate::{
  util::Key,
  resources::{DependencyGraph, InputState, SketchEvent, Geometry},
  components::*,
};

pub struct RemoveGeomSystem;

impl<'a> System<'a> for RemoveGeomSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, InputState>,
    Write<'a, DependencyGraph>,
    Write<'a, EventChannel<SketchEvent>>,
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
    mut sketch_events,
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
      }

      // Remove everything
      for entity in to_remove {

        // Remove the thing from
        points.remove(entity);
        let maybe_sym_pt = sym_points.remove(entity);
        let maybe_pt_sty = point_styles.remove(entity);
        lines.remove(entity);
        let maybe_sym_ln = sym_lines.remove(entity);
        let maybe_ln_sty = line_styles.remove(entity);
        selected.remove(entity);

        // Push the event
        if let Some(sym_pt) = maybe_sym_pt {
          if let Some(pt_sty) = maybe_pt_sty {
            sketch_events.single_write(SketchEvent::Removed(entity, Geometry::Point(sym_pt, pt_sty)));
          }
        }
        if let Some(sym_ln) = maybe_sym_ln {
          if let Some(ln_sty) = maybe_ln_sty {
            sketch_events.single_write(SketchEvent::Removed(entity, Geometry::Line(sym_ln, ln_sty)));
          }
        }
      }
    }
  }
}