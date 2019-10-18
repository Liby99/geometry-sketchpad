use super::{Vector2, Line, LineType, Circle, AABB, Project};

pub trait Intersect<T> {
  type Output;
  fn intersect(self, other: T) -> Self::Output;
}

static LINE_ITSCT_THRESHOLD : f64 = 1e-10;

fn itsct_line_is_not_none(l: Line, itsct: Vector2) -> bool {
  let d = (itsct - l.origin).dot(l.direction);
  match l.line_type {
    LineType::Line => true,
    LineType::Ray => d > -LINE_ITSCT_THRESHOLD,
    LineType::Segment(t) => d > -LINE_ITSCT_THRESHOLD && d < t + LINE_ITSCT_THRESHOLD,
  }
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

      // The intersection position
      let itsct = vec2![x_nom / det, y_nom / det];

      // Need to check the line type to make sure that intersection is on the line
      if !itsct_line_is_not_none(self, itsct) { return None };
      if !itsct_line_is_not_none(other, itsct) { return None };

      // If not none, then return the intersection
      Some(itsct)
    }
  }
}

#[cfg(test)]
mod test_line_line_intersection {
  use super::*;

  #[test]
  fn test_line_line_intersect_1() {
    let l1 = Line { origin: vec2![0.0, 0.0], direction: vec2![1.0, 0.0], line_type: LineType::Line };
    let l2 = Line { origin: vec2![-1.0, -1.0], direction: vec2![0.0, 1.0], line_type: LineType::Line };
    assert!(l1.intersect(l2) == Some(vec2![-1.0, 0.0]));
  }

  #[test]
  fn test_line_line_intersect_2() {
    let l1 = Line { origin: vec2![0.0, 0.0], direction: vec2![1.0, 0.0], line_type: LineType::Ray };
    let l2 = Line { origin: vec2![-1.0, -1.0], direction: vec2![0.0, 1.0], line_type: LineType::Line };
    assert!(l1.intersect(l2).is_none());
  }

  #[test]
  fn test_line_line_intersect_3() {
    let l1 = Line { origin: vec2![0.0, 0.0], direction: vec2![1.0, 0.0], line_type: LineType::Segment(0.5) };
    let l2 = Line { origin: vec2![1.0, -1.0], direction: vec2![0.0, 1.0], line_type: LineType::Line };
    assert!(l1.intersect(l2).is_none());
  }

  #[test]
  fn test_line_line_intersect_4() {
    let l1 = Line { origin: vec2![0.0, 0.0], direction: vec2![1.0, 0.0], line_type: LineType::Segment(1.5) };
    let l2 = Line { origin: vec2![1.0, -1.0], direction: vec2![0.0, 1.0], line_type: LineType::Line };
    assert!(l1.intersect(l2) == Some(vec2![1.0, 0.0]));
  }
}

impl Intersect<AABB> for Line {
  type Output = Option<(Vector2, Vector2)>;

  fn intersect(self, aabb: AABB) -> Self::Output {
    let AABB { x: x_min, y: y_min, width, height } = aabb;
    let x_max = x_min + width;
    let y_max = y_min + height;
    let Line { origin: Vector2 { x: ox, y: oy }, direction: Vector2 { x: dx, y: dy }, line_type } = self;
    let (p1, p2) = if dx == 0.0 {
      if x_min <= ox && ox <= x_max {
        (vec2![ox, y_min], vec2![ox, y_max])
      } else {
        return None
      }
    } else if dy == 0.0 {
      if y_min <= oy && oy <= y_max {
        (vec2![x_min, oy], vec2![x_max, oy])
      } else {
        return None
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
        (true, true, false, false) => (top, right),
        (true, false, true, false) => (top, bottom),
        (true, false, false, true) => (top, left),
        (false, true, true, false) => (right, bottom),
        (false, true, false, true) => (right, left),
        (false, false, true, true) => (bottom, left),
        _ => return None
      }
    };
    let d1 = (p1 - self.origin).dot(self.direction) > 0.0;
    let d2 = (p2 - self.origin).dot(self.direction) > 0.0;
    match line_type {
      LineType::Line => Some((p1, p2)),
      LineType::Ray => {
        if d1 && d2 {
          Some((p1, p2))
        } else if d1 {
          Some((self.origin, p1))
        } else if d2 {
          Some((self.origin, p2))
        } else {
          None
        }
      },
      LineType::Segment(t) => {
        let (a, b) = (self.origin, self.origin + self.direction * t);
        let (ca, cb) = (aabb.contains(a), aabb.contains(b));
        if ca && cb {
          Some((a, b))
        } else if ca {
          if d1 { Some((p1, a)) } else if d2 { Some((p2, a)) } else { None }
        } else if cb {
          if d1 { Some((p1, b)) } else if d2 { Some((p2, b)) } else { None }
        } else {
          Some((p1, p2))
        }
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

pub enum CircleIntersect {
  TwoPoints(Vector2, Vector2),
  OnePoint(Vector2),
  None,
}

static CIRCLE_ITSCT_THRESHOLD : f64 = 1e-5;

impl Intersect<Line> for Circle {
  type Output = CircleIntersect;

  fn intersect(self, line: Line) -> Self::Output {
    let proj = self.center.project(line);
    let dist = (proj - self.center).magnitude();
    if dist < self.radius - CIRCLE_ITSCT_THRESHOLD {
      let da = (self.radius * self.radius - dist * dist).sqrt();
      let t_proj = (proj - line.origin).dot(line.direction);
      let p1 = line.origin + line.direction * (t_proj - da);
      let p2 = line.origin + line.direction * (t_proj + da);
      let has_p1 = itsct_line_is_not_none(line, p1);
      let has_p2 = itsct_line_is_not_none(line, p2);
      if has_p1 && has_p2 {
        CircleIntersect::TwoPoints(p1, p2)
      } else if has_p1 {
        CircleIntersect::OnePoint(p1)
      } else if has_p2 {
        CircleIntersect::OnePoint(p2)
      } else {
        CircleIntersect::None
      }
    } else if (dist - self.radius).abs() < CIRCLE_ITSCT_THRESHOLD {
      CircleIntersect::OnePoint(proj)
    } else {
      CircleIntersect::None
    }
  }
}

impl Intersect<Circle> for Line {
  type Output = CircleIntersect;

  fn intersect(self, circle: Circle) -> Self::Output {
    circle.intersect(self)
  }
}

impl Intersect<Circle> for Circle {
  type Output = CircleIntersect;

  fn intersect(self, other: Circle) -> Self::Output {
    let center_diff = other.center - self.center;
    let d = center_diff.magnitude();
    if d < self.radius + other.radius - CIRCLE_ITSCT_THRESHOLD {
      let center_theta = center_diff.y.atan2(center_diff.x);
      let d1 = (d * d - other.radius * other.radius + self.radius * self.radius) / (2.0 * d);
      let theta = (d1 / self.radius).acos();
      let theta_1 = center_theta - theta;
      let theta_2 = center_theta + theta;
      let p1 = self.center + vec2![self.radius * theta_1.cos(), self.radius * theta_1.sin()];
      let p2 = self.center + vec2![self.radius * theta_2.cos(), self.radius * theta_2.sin()];
      CircleIntersect::TwoPoints(p1, p2)
    } else if d < self.radius + other.radius + CIRCLE_ITSCT_THRESHOLD {
      CircleIntersect::OnePoint(self.center + center_diff / d * self.radius)
    } else {
      CircleIntersect::None
    }
  }
}