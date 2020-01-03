use super::super::*;

pub trait Intersect<T> {
  type Output;
  fn intersect(self, other: T) -> Self::Output;
}

static LINE_ITSCT_THRESHOLD: f64 = 1e-10;

fn itsct_line_is_not_none(l: Line, itsct: Vector2) -> bool {
  let t = l.t_of_point(itsct);
  match l.line_type {
    LineType::Straight => true,
    LineType::Ray => t > -LINE_ITSCT_THRESHOLD,
    LineType::Segment => t > -LINE_ITSCT_THRESHOLD && t < l.from_to_length() + LINE_ITSCT_THRESHOLD,
  }
}

impl Intersect<Line> for Line {
  type Output = Option<Vector2>;

  fn intersect(self, other: Self) -> Self::Output {
    let Vector2 { x: sox, y: soy } = self.from;
    let Vector2 { x: oox, y: ooy } = other.from;
    let Vector2 { x: sdx, y: sdy } = self.direction();
    let Vector2 { x: odx, y: ody } = other.direction();
    let det = sdx * ody - sdy * odx;
    if det == 0. {
      None
    } else {
      let x2 = sox + sdx;
      let y2 = soy + sdy;
      let x4 = oox + odx;
      let y4 = ooy + ody;
      let nom_1 = soy * x2 - sox * y2;
      let nom_2 = ooy * x4 - oox * y4;
      let x_nom = nom_1 * odx - sdx * nom_2;
      let y_nom = nom_1 * ody - sdy * nom_2;

      // The intersection position
      let itsct = vec2![x_nom / det, y_nom / det];

      // Need to check the line type to make sure that intersection is on the line
      if !itsct_line_is_not_none(self, itsct) {
        return None;
      };
      if !itsct_line_is_not_none(other, itsct) {
        return None;
      };

      // If not none, then return the intersection
      Some(itsct)
    }
  }
}

impl Intersect<AABB> for Line {
  type Output = Option<(Vector2, Vector2)>;

  fn intersect(self, aabb: AABB) -> Self::Output {
    let AABB {
      x: x_min,
      y: y_min,
      width,
      height,
    } = aabb;
    let x_max = x_min + width;
    let y_max = y_min + height;
    let Line {
      from: Vector2 { x: ox, y: oy },
      line_type,
      ..
    } = self;
    let Vector2 { x: dx, y: dy } = self.direction();
    let (p1, p2) = if dx == 0.0 {
      if x_min <= ox && ox <= x_max {
        (vec2![ox, y_min], vec2![ox, y_max])
      } else {
        return None;
      }
    } else if dy == 0.0 {
      if y_min <= oy && oy <= y_max {
        (vec2![x_min, oy], vec2![x_max, oy])
      } else {
        return None;
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
        _ => return None,
      }
    };
    let d1 = self.point_is_on_line(p1);
    let d2 = self.point_is_on_line(p2);
    match line_type {
      LineType::Straight => Some((p1, p2)),
      LineType::Ray => {
        if d1 && d2 {
          Some((p1, p2))
        } else if d1 {
          Some((self.from, p1))
        } else if d2 {
          Some((self.from, p2))
        } else {
          None
        }
      }
      LineType::Segment => {
        let (a, b) = (self.from, self.to);
        let (ca, cb) = (aabb.contains(a), aabb.contains(b));
        if ca && cb {
          Some((a, b))
        } else if ca {
          if d1 {
            Some((p1, a))
          } else if d2 {
            Some((p2, a))
          } else {
            None
          }
        } else if cb {
          if d1 {
            Some((p1, b))
          } else if d2 {
            Some((p2, b))
          } else {
            None
          }
        } else {
          if d1 && d2 {
            Some((p1, p2))
          } else {
            None
          }
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
    if closest_dist <= self.radius && self.radius <= furthest_dist {
      Some(())
    } else {
      None
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum CircleIntersect {
  TwoPoints(Vector2, Vector2),
  OnePoint(Vector2),
  None,
}

static CIRCLE_ITSCT_THRESHOLD: f64 = 1e-5;

impl Intersect<Line> for Circle {
  type Output = CircleIntersect;

  fn intersect(self, line: Line) -> Self::Output {
    let proj = self.center.project(line);
    let dist = (proj - self.center).magnitude();
    if dist < self.radius - CIRCLE_ITSCT_THRESHOLD {
      let da = (self.radius * self.radius - dist * dist).sqrt();
      let t_proj = line.t_of_point(proj);
      let p1 = line.point_at_t(t_proj - da);
      let p2 = line.point_at_t(t_proj + da);
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
    let (c1, c2) = if self.center < other.center {
      (self, other)
    } else {
      (other, self)
    };
    let center_diff = c2.center - c1.center;
    let d = center_diff.magnitude();
    if d < c1.radius + c2.radius - CIRCLE_ITSCT_THRESHOLD {
      let center_theta = center_diff.y.atan2(center_diff.x);
      let d1 = (d * d - c2.radius * c2.radius + c1.radius * c1.radius) / (2.0 * d);
      let theta = (d1 / c1.radius).acos();
      let theta_1 = center_theta - theta;
      let theta_2 = center_theta + theta;
      let p1 = c1.center + vec2![c1.radius * theta_1.cos(), c1.radius * theta_1.sin()];
      let p2 = c1.center + vec2![c1.radius * theta_2.cos(), c1.radius * theta_2.sin()];
      CircleIntersect::TwoPoints(p1, p2)
    } else if d < c1.radius + c2.radius + CIRCLE_ITSCT_THRESHOLD {
      CircleIntersect::OnePoint(c1.center + center_diff / d * c1.radius)
    } else {
      CircleIntersect::None
    }
  }
}

impl Intersect<AABB> for AABB {
  type Output = Option<AABB>;

  fn intersect(self, other: AABB) -> Self::Output {
    let x_min = self.x_min().max(other.x_min());
    let x_max = self.x_max().min(other.x_max());
    let y_min = self.y_min().max(other.y_min());
    let y_max = self.y_max().min(other.y_max());
    if x_min <= x_max && y_min <= y_max {
      Some(AABB {
        x: x_min,
        y: y_min,
        width: x_max - x_min,
        height: y_max - y_min,
      })
    } else {
      None
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_hor_line_aabb_intersect() {
    let l = Line {
      from: vec2![-1.0, 0.0],
      to: vec2![1.0, 0.0],
      line_type: LineType::Straight,
    };
    let aabb = AABB {
      x: -0.5,
      y: -0.5,
      width: 1.0,
      height: 1.0,
    };
    assert!(l.intersect(aabb) == Some((vec2![-0.5, 0.0], vec2![0.5, 0.0])));
  }
}
