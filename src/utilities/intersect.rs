use super::{Vector2, Line, Circle, AABB, Project};

pub trait Intersect<T> {
  type Output;
  fn intersect(self, other: T) -> Self::Output;
}

impl Intersect<Line> for Line {
  type Output = Option<Vector2>;

  fn intersect(self, other: Self) -> Self::Output {
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
  type Output = Option<(Vector2, Vector2)>;

  fn intersect(self, AABB { x: x_min, y: y_min, width, height }: AABB) -> Self::Output {
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
        y_min <= left.y && left.y <= y_max,
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

impl Intersect<AABB> for Circle {
  type Output = Option<()>;

  fn intersect(self, aabb: AABB) -> Self::Output {
    let closest_dist = (aabb.get_closest_point_to(self.center) - self.center).magnitude();
    let furthest_dist = (aabb.get_furthest_point_to(self.center) - self.center).magnitude();
    if closest_dist <= self.radius && self.radius <= furthest_dist { Some(()) } else { None }
  }
}

pub enum CircleLineIntersect {
  TwoPoints(Vector2, Vector2),
  OnePoint(Vector2),
  None,
}

pub static CIRCLE_LINE_ITSCT_THRESHOLD : f64 = 1e-5;

impl Intersect<Line> for Circle {
  type Output = CircleLineIntersect;

  fn intersect(self, line: Line) -> Self::Output {
    let proj = self.center.project(line);
    let dist = (proj - self.center).magnitude();
    if dist < self.radius - CIRCLE_LINE_ITSCT_THRESHOLD {
      let da = (self.radius * self.radius - dist * dist).sqrt();
      let t_proj = (proj - line.origin).dot(line.direction);
      CircleLineIntersect::TwoPoints(
        line.origin + line.direction * (t_proj - da),
        line.origin + line.direction * (t_proj + da),
      )
    } else if (dist - self.radius).abs() < CIRCLE_LINE_ITSCT_THRESHOLD {
      CircleLineIntersect::OnePoint(proj)
    } else {
      CircleLineIntersect::None
    }
  }
}

impl Intersect<Circle> for Line {
  type Output = CircleLineIntersect;

  fn intersect(self, circle: Circle) -> Self::Output {
    circle.intersect(self)
  }
}