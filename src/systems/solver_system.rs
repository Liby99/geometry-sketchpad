use specs::prelude::*;
use crate::components::{
  point::{SymbolicPoint, Point},
  line::{SymbolicLine, Line},
};

enum ToCompute {
  Point(Entity),
  Line(Entity),
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
        ToCompute::Point(ent) => match points.get(ent) {
          Some(_) => (), // Ignore if already
          None => match sym_points.get(ent) {
            Some(sym) => match sym {
              SymbolicPoint::Free(pos) => {
                if let Err(err) = points.insert(ent, Point(pos.clone())) {
                  panic!("Error when inserting position: {:?}", err);
                }
              },
            },
            None => panic!("Could not find to compute point"),
          },
        },
        ToCompute::Line(ent) => match lines.get(ent) {
          Some(_) => (), // Ignore if already computed
          None => match sym_lines.get(ent) {
            Some(sym) => match sym {
              SymbolicLine::TwoPoints(p1_ent, p2_ent) => match points.get(*p1_ent) {
                Some(Point(pos_1)) => match points.get(*p2_ent) {
                  Some(Point(pos_2)) => {
                    if let Err(err) = lines.insert(ent, Line {
                      origin: pos_1.clone(),
                      direction: (pos_2.clone() - pos_1.clone()).normalized(),
                    }) {
                      panic!("Error when inserting line: {:?}", err);
                    }
                  },
                  None => {
                    stack.push(ToCompute::Line(ent)); // First push self
                    stack.push(ToCompute::Point(*p2_ent)); // Then push calculate p2 first
                  }
                },
                None => {
                  stack.push(ToCompute::Line(ent)); // First push self
                  stack.push(ToCompute::Point(*p1_ent)); // Then push calculate p2 first
                }
              }
            },
            None => panic!("Could not find to compute line"),
          },
        }
      }
    }
  }
}