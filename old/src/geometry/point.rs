use crate::util::Id;
use crate::math::Vec2;

#[derive(Clone, Copy)]
pub enum PointConstruct {
  Free { pos: Vec2 },
  OnLine { l: Id, t: f64 },
  LineLineIntersect { l1: Id, l2: Id },
}

pub type Point = Vec2;