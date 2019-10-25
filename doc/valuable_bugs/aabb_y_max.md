# AABB `y_max` method

## Before

``` rust
pub fn y_max(&self) -> f64 {
  self.y + self.width
}
```

## After

``` rust
pub fn y_max(&self) -> f64 {
  self.y + self.height
}
```