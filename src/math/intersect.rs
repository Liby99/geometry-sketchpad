use super::{Vector2, Line, AABB};

pub trait Intersect<T> {
  type Output;
  fn intersect(self, other: T) -> Option<Self::Output>;
}

impl Intersect<Line> for Line {
  type Output = Vector2;

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

impl Intersect<AABB> for Line {
  type Output = (Vector2, Vector2);

  fn intersect(self, AABB { x: x_min, y: y_min, width, height }: AABB) -> Option<Self::Output> {
    let x_max = x_min + width;
    let y_max = y_min + height;
    let Line { origin: Vector2 { x: ox, y: oy }, direction: Vector2 { x: dx, y: dy } } = self;
    if dx == 0.0 {
      if x_min <= ox && ox <= x_max {
        Some((vec2![ox, y_min], vec2![ox, y_max]))
      } else {
        None
      }
    } else if dy == 0.0 {
      if y_min <= oy && oy <= y_max {
        Some((vec2![x_min, oy], vec2![x_max, oy]))
      } else {
        None
      }
    } else {
      let top = vec2![ox + (y_max - oy) / dy * dx, y_max];
      let right = vec2![x_max, oy + (x_max - ox) / dx * dy];
      let bottom = vec2![ox + (y_min - oy) / dy * dx, y_min];
      let left = vec2![x_min, oy + (x_min - ox) / dx * dy];

      match (
        x_min <= top.x && top.x <= x_max,
        y_min <= right.y && right.y <= y_max,
        x_min <= bottom.x && bottom.x <= x_max,
        y_min <= left.y && left.y <= y_max
      ) {
        (true, true, false, false) => Some((top, right)),
        (true, false, true, false) => Some((top, bottom)),
        (true, false, false, true) => Some((top, left)),
        (false, true, true, false) => Some((right, bottom)),
        (false, true, false, true) => Some((right, left)),
        (false, false, true, true) => Some((bottom, left)),
        _ => None
      }
    }
  }
}