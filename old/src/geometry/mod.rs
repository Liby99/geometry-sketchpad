pub mod line;
pub mod point;

pub trait Intersect<T> {
  type Output;
  fn intersect(self, other: T) -> Option<Self::Output>;
}