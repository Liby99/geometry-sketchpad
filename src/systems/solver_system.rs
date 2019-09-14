use specs::prelude::*;
use crate::{
  math::Vector2,
  components::{
    point::{SymbolicPoint, Point},
    line::{SymbolicLine, Line},
  }
};

enum ToCompute {
  Point(Entity),
  Line(Entity),
}

enum SolveResult<T> {
  Solved(T), // The result of point
  AlreadyComputed, // Already Computed
  Request(ToCompute), // Need other dependency
}

fn insert_point<'a>(points: &mut WriteStorage<'a, Point>, ent: Entity, pos: Vector2) {
  if let Err(err) = points.insert(ent, Point(pos)) {
    panic!("Error when inserting position: {:?}", err);
  }
}

fn insert_line<'a>(lines: &mut WriteStorage<'a, Line>, ent: Entity, origin: Vector2, direction: Vector2) {
  if let Err(err) = lines.insert(ent, Line { origin, direction }) {
    panic!("Error when inserting position: {:?}", err);
  }
}

fn solve_point<'a>(
  sym_points: &ReadStorage<'a, SymbolicPoint>,
  points: &mut WriteStorage<'a, Point>,
  lines: &mut WriteStorage<'a, Line>,
  ent: Entity,
) -> SolveResult<Vector2> {
  match points.get(ent) {
    Some(_) => SolveResult::AlreadyComputed,
    None => match sym_points.get(ent) {
      Some(sym) => match sym {
        SymbolicPoint::Free(pos) => SolveResult::Solved(*pos),
        SymbolicPoint::OnLine(line_ent, t) => match lines.get(*line_ent) {
          Some(Line { origin, direction }) => {
            SolveResult::Solved(origin.clone() + *t * direction.clone())
          },
          None => SolveResult::Request(ToCompute::Line(*line_ent))
        }
      },
      None => panic!("Could not find to compute point"),
    },
  }
}

fn solve_line<'a>(
  sym_lines: &ReadStorage<'a, SymbolicLine>,
  points: &mut WriteStorage<'a, Point>,
  lines: &mut WriteStorage<'a, Line>,
  ent: Entity,
) -> SolveResult<(Vector2, Vector2)> {
  match lines.get(ent) {
    Some(_) => SolveResult::AlreadyComputed,
    None => match sym_lines.get(ent) {
      Some(sym) => match sym {
        SymbolicLine::TwoPoints(p1_ent, p2_ent) => match points.get(*p1_ent) {
          Some(Point(pos_1)) => match points.get(*p2_ent) {
            Some(Point(pos_2)) => {
              let origin = *pos_1;
              let direction = (*pos_2 - *pos_1).normalized();
              SolveResult::Solved((origin, direction))
            },
            None => SolveResult::Request(ToCompute::Point(*p2_ent))
          },
          None => SolveResult::Request(ToCompute::Point(*p1_ent))
        },
      },
      None => panic!("Could not find to compute line"),
    },
  }
}

pub struct SolverSystem;

impl<'a> System<'a> for SolverSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
    WriteStorage<'a, Point>,
    WriteStorage<'a, Line>,
  );

  fn run(&mut self, (
    entities,
    sym_points,
    sym_lines,
    mut points,
    mut lines,
  ): Self::SystemData) {
    let mut stack = vec![];

    points.clear();
    lines.clear();

    // Fisrt push all the lines into stack
    for (ent, _) in (&*entities, &sym_lines).join() {
      stack.push(ToCompute::Line(ent));
    }

    // Then push all the points into stack
    // As we want to first calculat points
    for (ent, _) in (&*entities, &sym_points).join() {
      stack.push(ToCompute::Point(ent));
    }

    // Calculate all the elements in the stack
    while !stack.is_empty() {
      let to_comp = stack.pop().unwrap();
      match to_comp {
        ToCompute::Point(ent) => match solve_point(&sym_points, &mut points, &mut lines, ent) {
          SolveResult::AlreadyComputed => (),
          SolveResult::Solved(p) => insert_point(&mut points, ent, p),
          SolveResult::Request(req) => {
            stack.push(ToCompute::Point(ent));
            stack.push(req);
          },
        },
        ToCompute::Line(ent) => match solve_line(&sym_lines, &mut points, &mut lines, ent) {
          SolveResult::AlreadyComputed => (),
          SolveResult::Solved((origin, direction)) => insert_line(&mut lines, ent, origin, direction),
          SolveResult::Request(req) => {
            stack.push(ToCompute::Point(ent));
            stack.push(req);
          },
        }
      }
    }
  }
}