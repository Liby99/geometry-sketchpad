use std::collections::HashSet;
use specs::prelude::*;
use crate::{
  utilities::{Vector2, Intersect, CircleIntersect, LineType},
  resources::{
    DependencyGraph,
    events::{SketchEvent, SketchEventChannel, SketchEventReader, SketchGeometry},
  },
  components::{SymbolicPoint, Point, SymbolicLine, Line, SymbolicCircle, Circle, CircleIntersectionType},
};

enum ToCompute {
  Point(Entity),
  Line(Entity),
  Circle(Entity),
}

impl ToCompute {
  fn get_entity(&self) -> &Entity {
    match self {
      ToCompute::Point(e) => e,
      ToCompute::Line(e) => e,
      ToCompute::Circle(e) => e,
    }
  }
}

enum SolveResult {
  AlreadyComputed, // Already Computed
  SolvedPoint(Point), // The result of point
  SolvedLine(Line), // The result of line
  SolvedCircle(Circle), // The result of circle
  Request(ToCompute), // Need other dependency
  Undefined, // The result does not exist
}

fn insert_point<'a>(points: &mut WriteStorage<'a, Point>, ent: Entity, p: Point) {
  if let Err(err) = points.insert(ent, p) { panic!(err) }
}

fn insert_line<'a>(lines: &mut WriteStorage<'a, Line>, ent: Entity, line: Line) {
  if let Err(err) = lines.insert(ent, line) { panic!(err) }
}

fn insert_circle<'a>(circles: &mut WriteStorage<'a, Circle>, ent: Entity, circle: Circle) {
  if let Err(err) = circles.insert(ent, circle) { panic!(err) }
}

fn solve_point<'a>(
  sym_points: &ReadStorage<'a, SymbolicPoint>,
  points: &mut WriteStorage<'a, Point>,
  lines: &mut WriteStorage<'a, Line>,
  circles: &mut WriteStorage<'a, Circle>,
  ent: Entity,
) -> SolveResult {

  // First check if the result is already computed
  match points.get(ent) {
    Some(_) => SolveResult::AlreadyComputed,
    None => match sym_points.get(ent) {
      Some(sym) => match sym {

        // If it is a free point, then the solved point is right there
        SymbolicPoint::Free(pos) => SolveResult::SolvedPoint(*pos),

        //
        SymbolicPoint::MidPoint(p1_ent, p2_ent) => match points.get(*p1_ent) {
          Some(point_1) => match points.get(*p2_ent) {
            Some(point_2) => SolveResult::SolvedPoint((*point_2 + *point_1) / 2.0),
            None => SolveResult::Request(ToCompute::Point(*p2_ent)),
          },
          None => SolveResult::Request(ToCompute::Point(*p1_ent)),
        },

        // If it is a point on a line, then the point is at distance t from origin
        // along the direction. If the computed line is not found we request the
        // algorithm to compute the line first
        SymbolicPoint::OnLine(line_ent, t) => match lines.get(*line_ent) {
          Some(Line { origin, direction, .. }) => SolveResult::SolvedPoint(origin.clone() + *t * *direction),
          None => SolveResult::Request(ToCompute::Line(*line_ent))
        },

        // We demand two lines
        SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => match lines.get(*l1_ent) {
          Some(line_1) => match lines.get(*l2_ent) {
            Some(line_2) => match line_1.intersect(*line_2) {
              Some(p) => SolveResult::SolvedPoint(p),
              None => SolveResult::Undefined,
            },
            None => SolveResult::Request(ToCompute::Line(*l2_ent)),
          },
          None => SolveResult::Request(ToCompute::Line(*l1_ent)),
        },

        // We demand circle to be drawn first
        SymbolicPoint::OnCircle(circ_ent, theta) => match circles.get(*circ_ent) {
          Some(circ) => {
            let pos = circ.center + vec2![theta.cos() * circ.radius, theta.sin() * circ.radius];
            SolveResult::SolvedPoint(pos)
          },
          None => SolveResult::Request(ToCompute::Circle(*circ_ent)),
        },

        //
        SymbolicPoint::CircleLineIntersect(circ_ent, line_ent, ty) => match circles.get(*circ_ent) {
          Some(c) => match lines.get(*line_ent) {
            Some(l) => match c.intersect(*l) {
              CircleIntersect::None => SolveResult::Undefined,
              CircleIntersect::OnePoint(p) => SolveResult::SolvedPoint(p),
              CircleIntersect::TwoPoints(p1, p2) => match ty {
                CircleIntersectionType::First => SolveResult::SolvedPoint(p1),
                CircleIntersectionType::Second => SolveResult::SolvedPoint(p2),
              },
            },
            None => SolveResult::Request(ToCompute::Line(*line_ent)),
          },
          None => SolveResult::Request(ToCompute::Circle(*circ_ent)),
        },

        SymbolicPoint::CircleCircleIntersect(c1_ent, c2_ent, ty) => match circles.get(*c1_ent) {
          Some(c1) => match circles.get(*c2_ent) {
            Some(c2) => match c1.intersect(*c2) {
              CircleIntersect::None => SolveResult::Undefined,
              CircleIntersect::OnePoint(p) => SolveResult::SolvedPoint(p),
              CircleIntersect::TwoPoints(p1, p2) => match ty {
                CircleIntersectionType::First => SolveResult::SolvedPoint(p1),
                CircleIntersectionType::Second => SolveResult::SolvedPoint(p2),
              },
            },
            None => SolveResult::Request(ToCompute::Circle(*c2_ent)),
          },
          None => SolveResult::Request(ToCompute::Circle(*c1_ent)),
        }
      },
      None => panic!("[solver_system] Could not find to compute point"),
    },
  }
}

fn solve_line<'a>(
  sym_lines: &ReadStorage<'a, SymbolicLine>,
  points: &mut WriteStorage<'a, Point>,
  lines: &mut WriteStorage<'a, Line>,
  ent: Entity,
) -> SolveResult {

  // First check the line is already computed
  match lines.get(ent) {
    Some(_) => SolveResult::AlreadyComputed,
    None => match sym_lines.get(ent) {
      Some(sym) => match sym {

        // If the line is constructed from two points, then we require the two
        // points to be computed first. After that the line is originated from
        // point 1 to the direction of point 2.
        SymbolicLine::TwoPoints(p1_ent, p2_ent) => match points.get(*p1_ent) {
          Some(pos_1) => match points.get(*p2_ent) {
            Some(pos_2) => {
              let origin = *pos_1;
              let direction = (*pos_2 - *pos_1).normalized();
              SolveResult::SolvedLine(Line { origin, direction, line_type: LineType::Line })
            },
            None => SolveResult::Request(ToCompute::Point(*p2_ent))
          },
          None => SolveResult::Request(ToCompute::Point(*p1_ent))
        },

        SymbolicLine::Ray(p1_ent, p2_ent) => match points.get(*p1_ent) {
          Some(pos_1) => match points.get(*p2_ent) {
            Some(pos_2) => {
              let origin = *pos_1;
              let direction = (*pos_2 - *pos_1).normalized();
              SolveResult::SolvedLine(Line { origin, direction, line_type: LineType::Ray })
            },
            None => SolveResult::Request(ToCompute::Point(*p2_ent))
          },
          None => SolveResult::Request(ToCompute::Point(*p1_ent))
        }

        SymbolicLine::Parallel(line_ent, point_ent) => match points.get(*point_ent) {
          Some(pos) => match lines.get(*line_ent) {
            Some(Line { direction, .. }) => SolveResult::SolvedLine(Line { origin: *pos, direction: *direction, line_type: LineType::Line }),
            None => SolveResult::Request(ToCompute::Line(*line_ent))
          },
          None => SolveResult::Request(ToCompute::Point(*point_ent))
        },

        SymbolicLine::Perpendicular(line_ent, point_ent) => match points.get(*point_ent) {
          Some(pos) => match lines.get(*line_ent) {
            Some(Line { direction: Vector2 { x, y }, .. }) => {
              let perp_dir = vec2![-y, *x];
              SolveResult::SolvedLine(Line { origin: *pos, direction: perp_dir, line_type: LineType::Line })
            },
            None => SolveResult::Request(ToCompute::Line(*line_ent))
          },
          None => SolveResult::Request(ToCompute::Point(*point_ent))
        },
      },
      None => panic!("[solver_system] Could not find to compute line"),
    },
  }
}

fn solve_circle<'a>(
  sym_circles: &ReadStorage<'a, SymbolicCircle>,
  points: &mut WriteStorage<'a, Point>,
  // lines: &mut WriteStorage<'a, Line>,
  circles: &mut WriteStorage<'a, Circle>,
  ent: Entity,
) -> SolveResult {
  match circles.get(ent) {
    Some(_) => SolveResult::AlreadyComputed,
    None => match sym_circles.get(ent) {
      Some(sym) => match sym {

        // If the line is constructed from two points, then we require the two
        // points to be computed first. After that the line is originated from
        // point 1 to the direction of point 2.
        SymbolicCircle::CenterRadius(p1_ent, p2_ent) => match points.get(*p1_ent) {
          Some(pos_1) => match points.get(*p2_ent) {
            Some(pos_2) => {
              let radius = (*pos_2 - *pos_1).magnitude();
              SolveResult::SolvedCircle(Circle::new(*pos_1, radius))
            },
            None => SolveResult::Request(ToCompute::Point(*p2_ent))
          },
          None => SolveResult::Request(ToCompute::Point(*p1_ent))
        },
      },
      None => panic!("[solver_system] Could not find to compute line"),
    },
  }
}

pub struct SolverSystem {
  need_initialize: bool,
  sketch_events_reader_id: Option<SketchEventReader>,
}

impl Default for SolverSystem {
  fn default() -> Self {
    Self {
      need_initialize: true,
      sketch_events_reader_id: None,
    }
  }
}

impl<'a> System<'a> for SolverSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, DependencyGraph>,
    Read<'a, SketchEventChannel>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, SymbolicCircle>,
    WriteStorage<'a, Point>,
    WriteStorage<'a, Line>,
    WriteStorage<'a, Circle>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.sketch_events_reader_id = Some(world.fetch_mut::<SketchEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    dependency_graph,
    sketch_events,
    sym_points,
    sym_lines,
    sym_circles,
    mut points,
    mut lines,
    mut circles,
  ): Self::SystemData) {
    let mut stack = vec![];
    let mut cannot_compute = HashSet::new();

    // Note: There are two crucial parts:
    //  1. Determine which entities to compute
    //  2. Compute
    // The algorithm is then basically
    //  1. Push all the entities to compute into the stack
    //  2. Solve the entities sequentially
    // For 1, we need
    //  - When starting up the program, we need all geometries get compute
    //    from scratch. So we push everything onto the stack.
    //  - Else, we read through the sketch events and make any changes
    //    - If inserted new, then just add that new thing to the stack
    //    - If updated, then push all descendents of that updated geom onto
    //      the stack
    //    - If removed, we don't really care since other algorithms should
    //      already removed all the descendents

    // This happens when starting up the program
    if self.need_initialize {
      self.need_initialize = false; // set to false afterwards

      // First push all the circles into stack
      for (ent, _) in (&*entities, &sym_circles).join() {
        circles.remove(ent);
        stack.push(ToCompute::Circle(ent));
      }

      // Then push all the lines into stack
      for (ent, _) in (&*entities, &sym_lines).join() {
        lines.remove(ent);
        stack.push(ToCompute::Line(ent));
      }

      // Finally push all the points into stack
      // As we want to first calculate points
      for (ent, _) in (&*entities, &sym_points).join() {
        points.remove(ent);
        stack.push(ToCompute::Point(ent));
      }
    } else {

      // In this else branch, we need to go through all the events
      if let Some(sketch_events_reader_id) = &mut self.sketch_events_reader_id {
        for event in sketch_events.read(sketch_events_reader_id) {
          match event {
            SketchEvent::Insert(entity, geom, _) => match geom {
              SketchGeometry::Point(_, _) => stack.push(ToCompute::Point(*entity)),
              SketchGeometry::Line(_, _) => stack.push(ToCompute::Line(*entity)),
              SketchGeometry::Circle(_, _) => stack.push(ToCompute::Circle(*entity)),
            },
            SketchEvent::Remove(_, _, _) => (), // Do nothing since they are already removed
            SketchEvent::Select(_) |
            SketchEvent::Deselect(_) => (), // Do nothing to select/deselect event
            SketchEvent::MovePoint(ent, _) => {
              let dependents = dependency_graph.get_all_dependents(ent);
              for dependent in dependents {
                if let Some(_) = sym_points.get(dependent) {
                  points.remove(dependent);
                  stack.push(ToCompute::Point(dependent));
                } else if let Some(_) = sym_lines.get(dependent) {
                  lines.remove(dependent);
                  stack.push(ToCompute::Line(dependent));
                } else if let Some(_) = sym_circles.get(dependent) {
                  circles.remove(dependent);
                  stack.push(ToCompute::Circle(dependent));
                }
              }
            },
            SketchEvent::Hide(_, _) |
            SketchEvent::Unhide(_, _) => (),
          }
        }
      } else {
        panic!("[solver_system] No sketch events reader id");
      }
    }

    // Calculate all the elements in the stack
    while !stack.is_empty() {
      let to_comp = stack.pop().unwrap();
      let (ent, result) = match to_comp {
        ToCompute::Point(ent) => (ent, solve_point(&sym_points, &mut points, &mut lines, &mut circles, ent)),
        ToCompute::Line(ent) => (ent, solve_line(&sym_lines, &mut points, &mut lines, ent)),
        ToCompute::Circle(ent) => (ent, solve_circle(&sym_circles, &mut points, &mut circles, ent)),
      };
      match result {
        SolveResult::AlreadyComputed => (),
        SolveResult::Undefined => {
          cannot_compute.insert(ent);
        },
        SolveResult::SolvedLine(l) => insert_line(&mut lines, ent, l),
        SolveResult::SolvedPoint(p) => insert_point(&mut points, ent, p),
        SolveResult::SolvedCircle(c) => insert_circle(&mut circles, ent, c),
        SolveResult::Request(req) => {
          if !cannot_compute.contains(req.get_entity()) {
            stack.push(to_comp);
            stack.push(req);
          }
        },
      }
    }
  }
}