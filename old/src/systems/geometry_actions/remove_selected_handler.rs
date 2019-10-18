use std::collections::HashSet;
use specs::prelude::*;
use crate::{
  resources::{
    DependencyGraph,
    events::{
      GeometryAction, GeometryActionReader, GeometryActionChannel,
      SketchEvent, SketchEventChannel, SketchGeometry,
    },
  },
  components::{SymbolicLine, SymbolicPoint, SymbolicCircle, PointStyle, LineStyle, CircleStyle, Selected},
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
    ReadStorage<'a, SymbolicCircle>,
    ReadStorage<'a, CircleStyle>,
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
    sym_circles,
    circle_styles,
    selected,
  ): Self::SystemData) {
    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_action_channel.read(reader_id) {
        match event {
          GeometryAction::RemoveSelected => {

            // Temporary cache
            let mut to_remove = HashSet::new();

            // Add all the selected stuff to stack
            for (entity, _) in (&entities, &selected).join() {
              to_remove.extend(dep_graph.get_all_dependents(&entity));
            }

            // Remove everything
            for entity in to_remove {

              // Push the event
              if let Some(sym_pt) = sym_points.get(entity) {
                if let Some(pt_sty) = point_styles.get(entity) {
                  sketch_events.single_write(SketchEvent::remove(entity, SketchGeometry::Point(*sym_pt, *pt_sty)));
                } else {
                  panic!("[remove_selected_handler] Cannot find point style for point entity {:?}", entity);
                }
              } else if let Some(sym_ln) = sym_lines.get(entity) {
                if let Some(ln_sty) = line_styles.get(entity) {
                  sketch_events.single_write(SketchEvent::remove(entity, SketchGeometry::Line(*sym_ln, *ln_sty)));
                } else {
                  panic!("[remove_selected_handler] Cannot find line style for line entity {:?}", entity);
                }
              } else if let Some(sym_cr) = sym_circles.get(entity) {
                if let Some(cr_sty) = circle_styles.get(entity) {
                  sketch_events.single_write(SketchEvent::remove(entity, SketchGeometry::Circle(*sym_cr, *cr_sty)));
                } else {
                  panic!("[remove_selected_handler] Cannot find line style for circle entity {:?}", entity);
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