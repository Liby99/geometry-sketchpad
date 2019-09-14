use specs::prelude::*;
use crate::{
  components::{
    point::{SymbolicPoint, Point},
    line::{SymbolicLine, Line},
  }
};

enum ToCompute {
  Point(Entity),
  Line(Entity),
}

enum SolveResult {
  AlreadyComputed, // Already Computed
  SolvedPoint(Point), // The result of point
  SolvedLine(Line), // The result of line
  Request(ToCompute), // Need other dependency
}

fn insert_point<'a>(points: &mut WriteStorage<'a, Point>, ent: Entity, p: Point) {
  if let Err(err) = points.insert(ent, p) {
    panic!("Error when inserting position: {:?}", err);
  }
}

fn insert_line<'a>(lines: &mut WriteStorage<'a, Line>, ent: Entity, line: Line) {
  if let Err(err) = lines.insert(ent, line) {
    panic!("Error when inserting position: {:?}", err);
  }
}

fn solve_point<'a>(
  sym_points: &ReadStorage<'a, SymbolicPoint>,
  points: &mut WriteStorage<'a, Point>,
  lines: &mut WriteStorage<'a, Line>,
  ent: Entity,
) -> SolveResult {
  match points.get(ent) {
    Some(_) => SolveResult::AlreadyComputed,
    None => match sym_points.get(ent) {
      Some(sym) => match sym {
        SymbolicPoint::Free(pos) => SolveResult::SolvedPoint(Point(*pos)),
        SymbolicPoint::OnLine(line_ent, t) => match lines.get(*line_ent) {
          Some(Line { origin, direction }) => {
            SolveResult::SolvedPoint(Point(origin.clone() + *t * direction.clone()))
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
) -> SolveResult {
  match lines.get(ent) {
    Some(_) => SolveResult::AlreadyComputed,
    None => match sym_lines.get(ent) {
      Some(sym) => match sym {
        SymbolicLine::TwoPoints(p1_ent, p2_ent) => match points.get(*p1_ent) {
          Some(Point(pos_1)) => match points.get(*p2_ent) {
            Some(Point(pos_2)) => {
              let origin = *pos_1;
              let direction = (*pos_2 - *pos_1).normalized();
              SolveResult::SolvedLine(Line { origin, direction })
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
      let (ent, result) = match to_comp {
        ToCompute::Point(ent) => (ent, solve_point(&sym_points, &mut points, &mut lines, ent)),
        ToCompute::Line(ent) => (ent, solve_line(&sym_lines, &mut points, &mut lines, ent)),
      };
      match result {
        SolveResult::AlreadyComputed => (),
        SolveResult::SolvedLine(l) => insert_line(&mut lines, ent, l),
        SolveResult::SolvedPoint(p) => insert_point(&mut points, ent, p),
        SolveResult::Request(req) => {
          stack.push(to_comp);
          stack.push(req);
        },
      }
    }
  }
}