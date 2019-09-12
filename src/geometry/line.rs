use crate::storage::Id;
use crate::math::Vec2;

pub enum LineConstruct {
  TwoPoint { p1: Id, p2: Id },
  Parallel { l: Id, p: Id },
}

pub struct Line {
  origin: Vec2,
  direction: Vec2,
}