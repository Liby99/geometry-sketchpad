use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::{
  resources::{DependencyGraph, SketchEvent, Geometry},
  components::{SymbolicLine, SymbolicPoint},
};

pub struct DependencyGraphCache {
  initialized: bool,
  sketch_events_reader_id: Option<ReaderId<SketchEvent>>,
}

impl Default for DependencyGraphCache {
  fn default() -> Self {
    Self {
      initialized: false,
      sketch_events_reader_id: None,
    }
  }
}

fn add_point(dependency_graph: &mut DependencyGraph, ent: &Entity, sym_point: &SymbolicPoint) {
  match sym_point {
    SymbolicPoint::Free(_) => (),
    SymbolicPoint::OnLine(line_ent, _) => {
      dependency_graph.add(line_ent, ent);
    },
    SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => {
      dependency_graph.add(l1_ent, ent);
      dependency_graph.add(l2_ent, ent);
    },
  }
}

fn add_line(dependency_graph: &mut DependencyGraph, ent: &Entity, sym_line: &SymbolicLine) {
  match sym_line {
    SymbolicLine::TwoPoints(p1_ent, p2_ent) => {
      dependency_graph.add(p1_ent, ent);
      dependency_graph.add(p2_ent, ent);
    },
    SymbolicLine::Parallel(line_ent, point_ent) => {
      dependency_graph.add(line_ent, ent);
      dependency_graph.add(point_ent, ent);
    },
  }
}

impl<'a> System<'a> for DependencyGraphCache {
  type SystemData = (
    Entities<'a>,
    Read<'a, EventChannel<SketchEvent>>,
    Write<'a, DependencyGraph>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.sketch_events_reader_id = Some(world.fetch_mut::<EventChannel<SketchEvent>>().register_reader());
  }

  fn run(&mut self, (
    entities,
    sketch_events,
    mut dependency_graph,
    sym_points,
    sym_lines,
  ): Self::SystemData) {
    if self.initialized {
      if let Some(reader_id) = &mut self.sketch_events_reader_id {
        for event in sketch_events.read(reader_id) {
          match event {
            SketchEvent::Inserted(entity, geom) => match geom {
              Geometry::Point(sym_point, _) => add_point(&mut dependency_graph, entity, sym_point),
              Geometry::Line(sym_line, _) => add_line(&mut dependency_graph, entity, sym_line),
            },
            SketchEvent::Removed(entity, _) => dependency_graph.remove(entity),
          }
        }
      } else {
        panic!("[dependency_graph_cache] No sketch event reader id");
      }
    } else {
      for (entity, sym_point) in (&entities, &sym_points).join() {
        add_point(&mut dependency_graph, &entity, sym_point);
      }
      for (entity, sym_line) in (&entities, &sym_lines).join() {
        add_line(&mut dependency_graph, &entity, sym_line);
      }
    }
  }
}