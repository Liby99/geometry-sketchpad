use crate::util::Id;
use crate::math::Vector2;

#[derive(Clone, Copy)]
pub enum PointConstruct {
  Free { pos: Vector2 },
  OnLine { l: Id, t: f64 },
  LineLineIntersect { l1: Id, l2: Id },
}

pub type Point = Vector2;