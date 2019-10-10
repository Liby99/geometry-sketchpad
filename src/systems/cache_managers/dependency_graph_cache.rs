use specs::prelude::*;
use crate::{
  resources::{
    DependencyGraph,
    events::{SketchGeometry, SketchEvent, SketchEventChannel, SketchEventReader},
  },
  components::{SymbolicLine, SymbolicPoint, SymbolicCircle},
};

pub struct DependencyGraphCache {
  initialized: bool,
  sketch_events_reader_id: Option<SketchEventReader>,
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
    SymbolicPoint::MidPoint(p1_ent, p2_ent) => {
      dependency_graph.add(p1_ent, ent);
      dependency_graph.add(p2_ent, ent);
    },
    SymbolicPoint::OnLine(line_ent, _) => {
      dependency_graph.add(line_ent, ent);
    },
    SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => {
      dependency_graph.add(l1_ent, ent);
      dependency_graph.add(l2_ent, ent);
    },
    SymbolicPoint::OnCircle(circ_ent, _) => {
      dependency_graph.add(circ_ent, ent);
    },
    SymbolicPoint::CircleLineIntersect(circ_ent, line_ent, _) => {
      dependency_graph.add(circ_ent, ent);
      dependency_graph.add(line_ent, ent);
    },
    SymbolicPoint::CircleCircleIntersect(c1_ent, c2_ent, _) => {
      dependency_graph.add(c1_ent, ent);
      dependency_graph.add(c2_ent, ent);
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
    SymbolicLine::Perpendicular(line_ent, point_ent) => {
      dependency_graph.add(line_ent, ent);
      dependency_graph.add(point_ent, ent);
    },
  }
}

fn add_circle(dependency_graph: &mut DependencyGraph, ent: &Entity, sym_circle: &SymbolicCircle) {
  match sym_circle {
    SymbolicCircle::CenterRadius(center_ent, point_on_circle) => {
      dependency_graph.add(center_ent, ent);
      dependency_graph.add(point_on_circle, ent);
    },
  }
}

impl<'a> System<'a> for DependencyGraphCache {
  type SystemData = (
    Entities<'a>,
    Read<'a, SketchEventChannel>,
    Write<'a, DependencyGraph>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, SymbolicCircle>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.sketch_events_reader_id = Some(world.fetch_mut::<SketchEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    sketch_events,
    mut dependency_graph,
    sym_points,
    sym_lines,
    sym_circles,
  ): Self::SystemData) {
    if self.initialized {
      if let Some(reader_id) = &mut self.sketch_events_reader_id {
        for event in sketch_events.read(reader_id) {
          match event {
            SketchEvent::Insert(entity, geom, _) => match geom {
              SketchGeometry::Point(sym_point, _) => add_point(&mut dependency_graph, entity, sym_point),
              SketchGeometry::Line(sym_line, _) => add_line(&mut dependency_graph, entity, sym_line),
              SketchGeometry::Circle(sym_circle, _) => add_circle(&mut dependency_graph, entity, sym_circle),
            },
            SketchEvent::Remove(entity, _, _) => dependency_graph.remove(entity),
            SketchEvent::Select(_) |
            SketchEvent::Deselect(_) |
            SketchEvent::MovePoint(_, _) |
            SketchEvent::Hide(_, _) |
            SketchEvent::Unhide(_, _) => (),
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
      for (entity, sym_circle) in (&entities, &sym_circles).join() {
        add_circle(&mut dependency_graph, &entity, sym_circle);
      }
    }
  }
}