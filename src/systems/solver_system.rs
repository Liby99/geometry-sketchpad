use specs::prelude::*;
use crate::components::point::{SymbolicPoint, Point};

enum ToCompute {
  Point(Entity),
}

pub struct SolverSystem;

impl<'a> System<'a> for SolverSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, SymbolicPoint>,
    WriteStorage<'a, Point>,
  );

  fn run(&mut self, (entities, sym_points, mut points): Self::SystemData) {
    let mut stack = vec![];

    // First push all the points into stack
    for (ent, _) in (&*entities, &sym_points).join() {
      stack.push(ToCompute::Point(ent));
    }

    // Calculate all the elements in the stack
    while !stack.is_empty() {
      let to_comp = stack.pop().unwrap();
      match to_comp {
        ToCompute::Point(ent) => match sym_points.get(ent) {
          Some(sym) => match sym {
            SymbolicPoint::Free(pos) => {
              if let Err(err) = points.insert(ent, Point(pos.clone())) {
                panic!("Error when inserting position: {:?}", err);
              }
            },
          },
          None => panic!("Could not find to compute point"),
        }
      }
    }
  }
}