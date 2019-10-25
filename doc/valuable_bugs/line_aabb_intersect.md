# Line AABB Intersect

Date: Oct 25, 2019

## Before

``` rust
let d1 = self.point_is_on_line(p1);
let d2 = self.point_is_on_line(p2);
let (a, b) = (self.from, self.to);
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
```

## After

``` rust
let d1 = self.point_is_on_line(p1);
let d2 = self.point_is_on_line(p2);
let (a, b) = (self.from, self.to);
let (ca, cb) = (aabb.contains(a), aabb.contains(b));
if ca && cb {
  Some((a, b))
} else if ca {
  if d1 { Some((p1, a)) } else if d2 { Some((p2, a)) } else { None }
} else if cb {
  if d1 { Some((p1, b)) } else if d2 { Some((p2, b)) } else { None }
} else {
  if d1 && d2 {
    Some((p1, p2))
  } else {
    None
  }
}
```