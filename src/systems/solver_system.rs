use specs::prelude::*;
use crate::{
  util::Intersect,
  // resources::DirtyState,
  components::{SymbolicPoint, Point, SymbolicLine, Line},
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
  Undefined, // The result does not exist
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

  // First check if the result is already computed
  match points.get(ent) {
    Some(_) => SolveResult::AlreadyComputed,
    None => match sym_points.get(ent) {
      Some(sym) => match sym {

        // If it is a free point, then the solved point is right there
        SymbolicPoint::Free(pos) => SolveResult::SolvedPoint(*pos),

        // If it is a point on a line, then the point is at distance t from origin
        // along the direction. If the computed line is not found we request the
        // algorithm to compute the line first
        SymbolicPoint::OnLine(line_ent, t) => match lines.get(*line_ent) {
          Some(Line { origin, direction }) => {
            SolveResult::SolvedPoint(origin.clone() + *t * direction.clone())
          },
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
              SolveResult::SolvedLine(Line { origin, direction })
            },
            None => SolveResult::Request(ToCompute::Point(*p2_ent))
          },
          None => SolveResult::Request(ToCompute::Point(*p1_ent))
        },

        SymbolicLine::Parallel(line_ent, point_ent) => match points.get(*point_ent) {
          Some(pos) => match lines.get(*line_ent) {
            Some(Line { direction, .. }) => SolveResult::SolvedLine(Line { origin: *pos, direction: *direction }),
            None => SolveResult::Request(ToCompute::Line(*line_ent))
          },
          None => SolveResult::Request(ToCompute::Point(*point_ent))
        }
      },
      None => panic!("Could not find to compute line"),
    },
  }
}

pub struct SolverSystem;

impl<'a> System<'a> for SolverSystem {
  type SystemData = (
    Entities<'a>,
    // Read<'a, DirtyState>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
    WriteStorage<'a, Point>,
    WriteStorage<'a, Line>,
  );

  fn run(&mut self, (
    entities,
    // dirty_state,
    sym_points,
    sym_lines,
    mut points,
    mut lines,
  ): Self::SystemData) {
    // if !dirty_state.is_solver_dirty {
    //   return;
    // }

    let mut stack = vec![];

    points.clear();
    lines.clear();

    // Fisrt push all the lines into stack
    for (ent, _) in (&*entities, &sym_lines).join() {
      stack.push(ToCompute::Line(ent));
    }

    // Then push all the points into stack
    // As we want to first calculate points
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
        SolveResult::Undefined => (),
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