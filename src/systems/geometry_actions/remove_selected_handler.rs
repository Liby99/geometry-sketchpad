use std::collections::HashSet;
use specs::prelude::*;
use crate::{
  resources::{
    DependencyGraph,
    events::{
      GeometryAction, GeometryActionReader, GeometryActionChannel,
      SketchEvent, SketchEventChannel, Geometry
    },
  },
  components::{SymbolicLine, SymbolicPoint, PointStyle, LineStyle, Selected},
};

pub struct RemoveSelectedHandler {
  geometry_action_reader: Option<GeometryActionReader>,
}

impl Default for RemoveSelectedHandler {
  fn default() -> Self {
    Self { geometry_action_reader: None }
  }
}

impl<'a> System<'a> for RemoveSelectedHandler {
  type SystemData = (
    Entities<'a>,
    Read<'a, GeometryActionChannel>,
    Read<'a, DependencyGraph>,
    Write<'a, SketchEventChannel>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, PointStyle>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, LineStyle>,
    ReadStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_action_reader = Some(world.fetch_mut::<GeometryActionChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    geometry_action_channel,
    dep_graph,
    mut sketch_events,
    sym_points,
    point_styles,
    sym_lines,
    line_styles,
    selected,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_action_channel.read(reader_id) {
        match event {
          GeometryAction::RemoveSelected => {

            // Temporary cache
            let mut to_remove = HashSet::new();
            let mut stack = vec![];

            // Add all the selected stuff to stack
            for (entity, _) in (&entities, &selected).join() {
              stack.push(entity);
            }

            // Explore the stack in dependency graph and put all of the children to `to_remove`
            while let Some(entity) = stack.pop() {
              to_remove.insert(entity);
              if let Some(children) = dep_graph.get(&entity) {
                for child in children {
                  stack.push(*child);
                }
              }
            }

            // Remove everything
            for entity in to_remove {

              // Remove the thing from
              let maybe_sym_pt = sym_points.get(entity);
              let maybe_pt_sty = point_styles.get(entity);
              let maybe_sym_ln = sym_lines.get(entity);
              let maybe_ln_sty = line_styles.get(entity);

              // Push the event
              if let Some(sym_pt) = maybe_sym_pt {
                if let Some(pt_sty) = maybe_pt_sty {
                  sketch_events.single_write(SketchEvent::Remove(entity, Geometry::Point(*sym_pt, *pt_sty)));
                }
              }
              if let Some(sym_ln) = maybe_sym_ln {
                if let Some(ln_sty) = maybe_ln_sty {
                  sketch_events.single_write(SketchEvent::Remove(entity, Geometry::Line(*sym_ln, *ln_sty)));
                }
              }
            }

            break;
          },
          _ => ()
        }
      }
    }
  }
}