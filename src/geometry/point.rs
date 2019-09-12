use crate::storage::Id;
use crate::math::Vec2;

pub enum PointConstruct {
  Free { pos: Vec2 },
  OnLine { l: Id, t: f64 },
  LineLineIntersect { l1: Id, l2: Id },
}

pub type Point = Vec2;