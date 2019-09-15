use crate::{
  math::Vector2,
  components::{Line, Point}
};

pub trait Intersect<T> {
  type Output;
  fn intersect(self, other: T) -> Option<Self::Output>;
}

impl Intersect<Line> for Line {
  type Output = Point;

  fn intersect(self, other: Self) -> Option<Self::Output> {
    let det = self.direction.x * other.direction.y - self.direction.y * other.direction.x;
    if det == 0. {
      None
    } else {
      let x2 = self.origin.x + self.direction.x;
      let y2 = self.origin.y + self.direction.y;
      let x4 = other.origin.x + other.direction.x;
      let y4 = other.origin.y + other.direction.y;
      let nom_1 = self.origin.y * x2 - self.origin.x * y2;
      let nom_2 = other.origin.y * x4 - other.origin.x * y4;
      let x_nom = nom_1 * other.direction.x - self.direction.x * nom_2;
      let y_nom = nom_1 * other.direction.y - self.direction.y * nom_2;
      Some(vec2![x_nom / det, y_nom / det])
    }
  }
}