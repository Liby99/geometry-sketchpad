use std::collections::BTreeMap;

use crate::{
  geometry::{
    line::{ Intersect, LineConstruct, Line },
    point::{ PointConstruct, Point },
  },
  storage::{ Id, Storage },
};

pub struct Context {
  pub points: Storage<PointConstruct>,
  pub lines: Storage<LineConstruct>,
}

pub enum SolveError {
  LineNotFound(Id),
  PointNotFound(Id),
}

pub enum ToCompute {
  Line(Id),
  Point(Id),
}

pub enum SolveResult<T> {
  Ok(Option<T>),
  Request(ToCompute),
  Err(SolveError)
}

impl Context {
  pub fn solve(&self) -> Solution {
    let mut sol = Solution::new();

    // TODO

    sol
  }

  fn solve_line(&self, sol: &Solution, line_id: Id) -> SolveResult<Line> {
    if sol.lines.contains_key(&line_id) {
      SolveResult::Ok(Some(sol.lines.get(&line_id).unwrap().clone()))
    } else {
      match self.lines.get(line_id) {
        Some(line_constr) => match *line_constr {

          // If a line is constructed by two points we demand the calculation
          // of both points
          LineConstruct::TwoPoint { p1, p2 } => match sol.points.get(&p1) {
            Some(point_1) => match sol.points.get(&p2) {
              Some(point_2) => {
                let line = Line {
                  origin: point_1.clone(),
                  direction: (*point_2 - *point_1).normalized(),
                };
                SolveResult::Ok(Some(line))
              },
              None => SolveResult::Request(ToCompute::Point(p2)),
            },
            None => SolveResult::Request(ToCompute::Point(p1)),
          }

          // If a line is constructed by parallel relation, we demand calculation
          // of both the point and the line
          LineConstruct::Parallel { l, p } => match sol.points.get(&p) {
            Some(point) => match sol.lines.get(&l) {
              Some(line) => {
                let parallel_line = Line {
                  origin: point.clone(),
                  direction: line.direction,
                };
                SolveResult::Ok(Some(parallel_line))
              },
              None => SolveResult::Request(ToCompute::Line(l)),
            },
            None => SolveResult::Request(ToCompute::Point(p)),
          }
        },
        None => SolveResult::Err(SolveError::LineNotFound(line_id)),
      }
    }
  }

  fn solve_point(&self, sol: &Solution, point_id: Id) -> SolveResult<Point> {
    if sol.points.contains_key(&point_id) {
      SolveResult::Ok(Some(sol.points.get(&point_id).unwrap().clone())) // Should not panic
    } else {
      match self.points.get(point_id) {
        Some(point_constr) => match *point_constr {

          // If it is a free point then directly get its position
          PointConstruct::Free { pos } => {
            SolveResult::Ok(Some(pos.clone()))
          },

          // If it is on a line, we demand the calculation of line first, otherwise
          // it will be of distance t along direction from origin
          PointConstruct::OnLine { l, t } => match sol.lines.get(&l) {
            Some(Line { origin, direction }) => {
              let p = *origin + *direction * t;
              SolveResult::Ok(Some(p))
            },
            None => SolveResult::Request(ToCompute::Line(l)),
          },

          // If it is on the intersection of two lines, we demand the calculation
          // of both lines first.
          PointConstruct::LineLineIntersect { l1, l2 } => match sol.lines.get(&l1) {
            Some(line_1) => match sol.lines.get(&l2) {
              Some(line_2) => SolveResult::Ok(line_1.intersect(line_2.clone())),
              None => SolveResult::Request(ToCompute::Line(l2)),
            },
            None => SolveResult::Request(ToCompute::Line(l1)),
          }
        },
        None => SolveResult::Err(SolveError::PointNotFound(point_id)),
      }
    }
  }
}

pub struct Solution {
  points: BTreeMap<Id, Point>,
  lines: BTreeMap<Id, Line>,
}

impl Solution {
  pub fn new() -> Self {
    Self {
      points: BTreeMap::new(),
      lines: BTreeMap::new(),
    }
  }
}