use super::super::{Vector2, Line, Circle};

pub trait Project<T> {
  type Output;
  fn project(self, target: T) -> Self::Output;
}

impl Project<Line> for Vector2 {
  type Output = Self;

  fn project(self, Line { origin, direction, .. }: Line) -> Self::Output {
    origin + (self - origin).dot(direction) * direction
  }
}

impl Project<Circle> for Vector2 {
  type Output = Self;

  fn project(self, Circle { center, radius }: Circle) -> Self::Output {
    center + (self - center).normalized() * radius
  }
}