use super::super::*;

pub trait Project<T> {
  type Output;
  fn project(self, target: T) -> Self::Output;
}

impl Project<Line> for Vector2 {
  type Output = Self;

  fn project(self, line: Line) -> Self::Output {
    let dir = line.direction();
    line.from + (self - line.from).dot(dir) * dir
  }
}

impl Project<Circle> for Vector2 {
  type Output = Self;

  fn project(self, Circle { center, radius }: Circle) -> Self::Output {
    center + (self - center).normalized() * radius
  }
}