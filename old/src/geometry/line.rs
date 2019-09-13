use crate::util::Id;
use crate::math::Vector2;

#[derive(Clone, Copy, Debug)]
pub enum LineConstruct {
  TwoPoint { p1: Id, p2: Id },
  Parallel { l: Id, p: Id },
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
  pub origin: Vector2,
  pub direction: Vector2,
}