use specs::prelude::*;
use crate::{events::*, resources::*, utilities::*, components::symbolics::*};

pub struct DependencyGraphManager {
  geometry_event_reader: Option<GeometryEventReader>,
}

impl Default for DependencyGraphManager {
  fn default() -> Self {
    Self { geometry_event_reader: None }
  }
}

impl<'a> System<'a> for DependencyGraphManager {
  type SystemData = (
    Read<'a, GeometryEventChannel>,
    Write<'a, DependencyGraph>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_event_reader = Some(world.fetch_mut::<GeometryEventChannel>().register_reader());
  }

  fn run(&mut self, (geometry_event_channel, mut dependency_graph): Self::SystemData) {
    if let Some(reader) = &mut self.geometry_event_reader {
      for event in geometry_event_channel.read(reader) {
        match event {
          GeometryEvent::Inserted(ent, geom, _) => match geom {
            Geometry::Point(sym_point, _) => insert_point(ent, sym_point, &mut *dependency_graph),
            Geometry::Line(sym_line, _) => insert_line(ent, sym_line, &mut *dependency_graph),
            Geometry::Circle(sym_circle, _) => insert_circle(ent, sym_circle, &mut *dependency_graph),
          },
          GeometryEvent::Removed(ent, _, _) => dependency_graph.remove(ent),
          GeometryEvent::Updated(_, _, _, _) => (),
        }
      }
    }
  }
}

fn insert_point(ent: &Entity, sym_point: &SymbolicPoint, dependency_graph: &mut DependencyGraph) {
  match sym_point {
    SymbolicPoint::Fixed(_) => (),
    SymbolicPoint::Free(_) => (),
    SymbolicPoint::MidPoint(p1_ent, p2_ent) => {
      dependency_graph.add(p1_ent, ent);
      dependency_graph.add(p2_ent, ent);
    },
    SymbolicPoint::OnLine(line_ent, _) => dependency_graph.add(line_ent, ent),
    SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => {
      dependency_graph.add(l1_ent, ent);
      dependency_graph.add(l2_ent, ent);
    },
    SymbolicPoint::OnCircle(circle_ent, _) => dependency_graph.add(circle_ent, ent),
    SymbolicPoint::CircleLineIntersect(circle_ent, line_ent, _) => {
      dependency_graph.add(circle_ent, ent);
      dependency_graph.add(line_ent, ent);
    },
    SymbolicPoint::CircleCircleIntersect(c1_ent, c2_ent, _) => {
      dependency_graph.add(c1_ent, ent);
      dependency_graph.add(c2_ent, ent);
    },
  }
}

fn insert_line(ent: &Entity, sym_line: &SymbolicLine, dependency_graph: &mut DependencyGraph) {
  match sym_line {
    SymbolicLine::Straight(p1_ent, p2_ent) => {
      dependency_graph.add(p1_ent, ent);
      dependency_graph.add(p2_ent, ent);
    },
    SymbolicLine::Ray(p1_ent, p2_ent) => {
      dependency_graph.add(p1_ent, ent);
      dependency_graph.add(p2_ent, ent);
    },
    SymbolicLine::Segment(p1_ent, p2_ent) => {
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

fn insert_circle(ent: &Entity, sym_circle: &SymbolicCircle, dependency_graph: &mut DependencyGraph) {
  match sym_circle {
    SymbolicCircle::CenterRadius(p1_ent, p2_ent) => {
      dependency_graph.add(p1_ent, ent);
      dependency_graph.add(p2_ent, ent);
    }
  }
}