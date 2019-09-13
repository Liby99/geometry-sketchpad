use crate::storage::Id;
use crate::geometry::point::Point;
use crate::math::Vec2;

pub trait Intersect<T> {
  type Output;
  fn intersect(self, other: T) -> Option<Self::Output>;
}

#[derive(Clone, Copy, Debug)]
pub enum LineConstruct {
  TwoPoint { p1: Id, p2: Id },
  Parallel { l: Id, p: Id },
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
  pub origin: Vec2,
  pub direction: Vec2,
}

impl Intersect<Line> for Line {
  type Output = Point;

  fn intersect(self, other: Self) -> Option<Self::Output> {
    if self.direction == other.direction {
      None
    } else {
      let diff_ori = other.origin - self.origin;
      let diff_dir = other.direction - self.direction;
      let t_x = -diff_ori.x / diff_dir.x;
      let p_self = self.origin + t_x * self.direction;
      Some(p_self)
    }
  }
}