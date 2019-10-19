use super::super::{Vector2, Position, Direction};

pub trait DotProduct<RHS> {
  fn dot(self, other: RHS) -> f64;
}

impl DotProduct<Vector2> for Vector2 {
  fn dot(self, other: Vector2) -> f64 {
    self.x * other.x + self.y * other.y
  }
}

impl DotProduct<Position> for Vector2 {
  fn dot(self, other: Position) -> f64 {
    self.dot(other.0)
  }
}

impl DotProduct<Direction> for Vector2 {
  fn dot(self, other: Direction) -> f64 {
    self.dot(other.0)
  }
}

impl DotProduct<Vector2> for Position {
  fn dot(self, other: Vector2) -> f64 {
    self.0.dot(other)
  }
}

impl DotProduct<Position> for Position {
  fn dot(self, other: Self) -> f64 {
    self.0.dot(other.0)
  }
}

impl DotProduct<Direction> for Position {
  fn dot(self, other: Direction) -> f64 {
    self.0.dot(other.0)
  }
}

impl DotProduct<Vector2> for Direction {
  fn dot(self, other: Vector2) -> f64 {
    self.0.dot(other)
  }
}

impl DotProduct<Position> for Direction {
  fn dot(self, other: Position) -> f64 {
    self.0.dot(other.0)
  }
}

impl DotProduct<Direction> for Direction {
  fn dot(self, other: Self) -> f64 {
    self.0.dot(other.0)
  }
}