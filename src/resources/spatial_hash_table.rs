use std::hash::Hash;
use itertools::Itertools;
use super::{Viewport, ViewportTransform};
use crate::math::{Vector2, Intersect};
use crate::components::{Point, Line};

static TILE_SIZE : f64 = 40.0;

#[derive(Debug)]
pub struct SpatialHashTable<T: Clone + Eq + Hash> {
  x_tiles: usize,
  y_tiles: usize,
  table: Vec<Vec<T>>,
}

pub type Tile = usize;

impl<T: Clone + Eq + Hash> Default for SpatialHashTable<T> {
  fn default() -> Self {
    Self { x_tiles: 0, y_tiles: 0, table: vec![] }
  }
}

impl<T: Clone + Eq + Hash> SpatialHashTable<T> {
  pub fn init_viewport(&mut self, vp: &Viewport) {
    self.x_tiles = (vp.actual_width() / TILE_SIZE).ceil() as usize;
    self.y_tiles = (vp.actual_height() / TILE_SIZE).ceil() as usize;
    self.table = vec![vec![]; self.x_tiles * self.y_tiles];
  }

  // p: point in virtual space
  pub fn insert_point(&mut self, ent: T, p: Point, vp: &Viewport) {
    if let Some(id) = self.get_cell(p.to_actual(vp)) {
      self.table[id].push(ent);
    }
  }

  /// l: line in virtual space
  pub fn insert_line(&mut self, ent: T, l: Line, vp: &Viewport) {
    let aabb = vp.actual_aabb();
    if let Some((p1, p2)) = l.to_actual(vp).intersect(aabb) {
      let (init_x_tile, init_y_tile) = self.get_unlimited_cell(p1);
      let (end_x_tile, end_y_tile) = self.get_unlimited_cell(p2);
      if init_x_tile == end_x_tile {
        if 0 <= init_x_tile && init_x_tile < self.x_tiles as i64 {
          for y_tile in 0..self.y_tiles {
            let tile = self.get_cell_by_x_y(init_x_tile as usize, y_tile);
            self.table[tile].push(ent.clone());
          }
        }
      } else if init_y_tile == end_y_tile {
        if 0 <= init_y_tile && init_y_tile < self.y_tiles as i64 {
          for x_tile in 0..self.x_tiles {
            let tile = self.get_cell_by_x_y(x_tile, init_y_tile as usize);
            self.table[tile].push(ent.clone());
          }
        }
      } else {
        // Making sure p1 to p2 is from left to right
        let (p1, p2) = if p1.x > p2.x { (p2, p1) } else { (p1, p2) };
        let dir = (p2 - p1).normalized();
        let p1 = p1 + dir;
        let (init_x_tile, init_y_tile) = self.get_unlimited_cell(p1);
        if dir.y < 0.0 {
          let mut curr_x = p1.x;
          let mut curr_y = p1.y;
          let mut curr_x_tile = init_x_tile as i64;
          let mut curr_y_tile = init_y_tile as i64;
          while 0 <= curr_x_tile && curr_x_tile < self.x_tiles as i64 && 0 <= curr_y_tile && curr_y_tile < self.y_tiles as i64 {
            let tile_offset_y = curr_y - curr_y_tile as f64 * TILE_SIZE;
            let next_y = curr_y_tile as f64 * TILE_SIZE;
            let next_x_diff = tile_offset_y / dir.y.abs() * dir.x;
            let next_x = curr_x + next_x_diff;
            let next_x_tile = (next_x / TILE_SIZE) as i64;
            for tile_x in curr_x_tile..(next_x_tile + 1) {
              if tile_x < self.x_tiles as i64 {
                let tile = self.get_cell_by_x_y(tile_x as usize, curr_y_tile as usize);
                self.table[tile].push(ent.clone());
              }
            }
            curr_x = next_x;
            curr_y = next_y;
            curr_x_tile = next_x_tile;
            curr_y_tile = curr_y_tile - 1;
          }
        } else {
          let mut curr_x = p1.x;
          let mut curr_y = p1.y;
          let mut curr_x_tile = init_x_tile as i64;
          let mut curr_y_tile = init_y_tile as i64;
          while 0 <= curr_x_tile && curr_x_tile < self.x_tiles as i64 && 0 <= curr_y_tile && curr_y_tile < self.y_tiles as i64 {
            let next_y = (curr_y_tile + 1) as f64 * TILE_SIZE;
            let offset_y = next_y - curr_y;
            let next_x_diff = offset_y / dir.y.abs() * dir.x;
            let next_x = curr_x + next_x_diff;
            let next_x_tile = (next_x / TILE_SIZE) as i64;
            for tile_x in curr_x_tile..(next_x_tile + 1) {
              if tile_x < self.x_tiles as i64 {
                let tile = self.get_cell_by_x_y(tile_x as usize, curr_y_tile as usize);
                self.table[tile].push(ent.clone());
              }
            }
            curr_x = next_x;
            curr_y = next_y;
            curr_x_tile = next_x_tile;
            curr_y_tile = curr_y_tile + 1;
          }
        }
      }
    }
  }

  /// p: point in actual space
  fn get_cell(&self, p: Point) -> Option<Tile> {
    let Vector2 { x, y } = p;
    let x_tile = (x / TILE_SIZE).floor();
    let y_tile = (y / TILE_SIZE).floor();
    if 0.0 <= x_tile && x_tile < self.x_tiles as f64 && 0.0 <= y_tile && y_tile < self.y_tiles as f64 {
      Some(self.get_cell_by_x_y(x_tile as usize, y_tile as usize))
    } else {
      None
    }
    // println!("(x: {}, x_tile: {}, y: {}, y_tile: {}, TILE_SIZE: {})", x, x_tile, y, y_tile, TILE_SIZE);
  }

  fn get_unlimited_cell(&self, p: Point) -> (i64, i64) {
    let Vector2 { x, y } = p;
    let x_tile = (x / TILE_SIZE).floor() as i64;
    let y_tile = (y / TILE_SIZE).floor() as i64;
    (x_tile, y_tile)
  }

  fn get_cell_by_x_y(&self, x_tile: usize, y_tile: usize) -> Tile {
    (y_tile * self.x_tiles) + x_tile
  }

  /// p: point in virtual space
  pub fn get_neighbor_entities(&self, p: Point, vp: &Viewport) -> Option<Vec<T>> {
    if let Some(center_tile) = self.get_cell(p.to_actual(vp)) {
      let mut tiles = vec![center_tile];

      let left = !self.is_left_border(center_tile);
      let right = !self.is_right_border(center_tile);
      let top = !self.is_top_border(center_tile);
      let bottom = !self.is_bottom_border(center_tile);

      if left { tiles.push(center_tile - 1) };
      if right { tiles.push(center_tile + 1) };
      if top { tiles.push(center_tile - self.x_tiles) };
      if bottom { tiles.push(center_tile + self.x_tiles) };
      if left && top { tiles.push(center_tile - self.x_tiles - 1) };
      if left && bottom { tiles.push(center_tile + self.x_tiles - 1) };
      if right && top { tiles.push(center_tile - self.x_tiles + 1) };
      if right && bottom { tiles.push(center_tile + self.x_tiles + 1) };

      Some(tiles.into_iter().map(|tile| self.table[tile].clone()).flatten().unique().collect())
    } else {
      None
    }
  }

  pub fn is_left_border(&self, tile: Tile) -> bool {
    tile % self.x_tiles == 0
  }

  pub fn is_right_border(&self, tile: Tile) -> bool {
    tile % self.x_tiles == self.x_tiles - 1
  }

  pub fn is_top_border(&self, tile: Tile) -> bool {
    tile / self.x_tiles < 1
  }

  pub fn is_bottom_border(&self, tile: Tile) -> bool {
    tile / self.x_tiles >= self.y_tiles - 1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_insert_point_1() {
    let vp = &Viewport::new(vec2![0., 0.], vec2![2., 2.], vec2![80., 80.]); // 田
    let mut table : SpatialHashTable<i32> = SpatialHashTable::default();
    table.init_viewport(vp);

    let p = vec2![0.0, 0.0];
    table.insert_point(0, p, vp);

    assert!(table.table[0].is_empty());
    assert!(table.table[1].is_empty());
    assert!(table.table[2].is_empty());
    assert!(table.table[3] == vec![0]);
  }

  #[test]
  fn test_insert_point_2() {
    let vp = &Viewport::new(vec2![0., 0.], vec2![2., 2.], vec2![80., 80.]); // 田
    let mut table : SpatialHashTable<i32> = SpatialHashTable::default();
    table.init_viewport(vp);

    let p = vec2![0.5, -0.5];
    table.insert_point(0, p, vp);

    assert!(table.table[0].is_empty());
    assert!(table.table[1].is_empty());
    assert!(table.table[2].is_empty());
    assert!(table.table[3] == vec![0]);
  }

  #[test]
  fn test_insert_line_1() {
    let vp = &Viewport::new(vec2![0., 0.], vec2![2., 2.], vec2![80., 80.]); // 田
    let mut table : SpatialHashTable<i32> = SpatialHashTable::default();
    table.init_viewport(vp);

    let l = Line { origin: vec2![-0.5, 0.0], direction: vec2![0.0, 1.0] };
    table.insert_line(0, l, vp);

    assert!(table.table[0] == vec![0]);
    assert!(table.table[1].is_empty());
    assert!(table.table[2] == vec![0]);
    assert!(table.table[3].is_empty());
  }

  #[test]
  fn test_insert_line_2() {
    let vp = &Viewport::new(vec2![0., 0.], vec2![2., 2.], vec2![80., 80.]); // 田
    let mut table : SpatialHashTable<i32> = SpatialHashTable::default();
    table.init_viewport(vp);

    let l = Line { origin: vec2![-0.5, 0.0], direction: vec2![(2.0 as f64).sqrt(), (2.0 as f64).sqrt()] };
    table.insert_line(0, l, vp);

    assert!(table.table[0] == vec![0]);
    assert!(table.table[1] == vec![0]);
    assert!(table.table[2] == vec![0]);
    assert!(table.table[3].is_empty());
  }

  /// + - - + - - +
  /// |     |     |
  /// | \   |     |
  /// + - - + - - +
  /// |    \|     |
  /// |     |\    |
  /// + - - + - - +
  #[test]
  fn test_insert_line_3() {
    let vp = &Viewport::new(vec2![0., 0.], vec2![2., 2.], vec2![80., 80.]); // 田
    let mut table : SpatialHashTable<i32> = SpatialHashTable::default();
    table.init_viewport(vp);

    let l = Line { origin: vec2![-0.5, 0.0], direction: vec2![(2.0 as f64).sqrt(), -(2.0 as f64).sqrt()] };
    table.insert_line(0, l, vp);

    assert!(table.table[0] == vec![0]);
    assert!(table.table[1].is_empty());
    assert!(table.table[2] == vec![0]);
    assert!(table.table[3] == vec![0]);
  }

  #[test]
  fn test_insert_line_4() {
    let vp = &Viewport::new(vec2![0., 0.], vec2![4., 4.], vec2![160., 160.]); // 田
    let mut table : SpatialHashTable<i32> = SpatialHashTable::default();
    table.init_viewport(vp);

    let l = Line { origin: vec2![-0.5, 0.0], direction: vec2![(2.0 as f64).sqrt(), (2.0 as f64).sqrt()] };
    table.insert_line(0, l, vp);

    println!("{:?}", table);

    for i in 0..16 {
      match i {
        2 | 3 | 5 | 6 | 8 | 9 | 12 => assert!(table.table[i] == vec![0]),
        _ => assert!(table.table[i].is_empty())
      }
    }
  }

  #[test]
  fn test_insert_line_5() {
    let vp = &Viewport::new(vec2![0., 0.], vec2![4., 4.], vec2![160., 160.]); // 田
    let mut table : SpatialHashTable<i32> = SpatialHashTable::default();
    table.init_viewport(vp);

    let sqrt17 = (17.0 as f64).sqrt();
    let l = Line { origin: vec2![0.0, 0.0], direction: vec2![4.0 / sqrt17, 1.0 / sqrt17] };
    table.insert_line(0, l, vp);

    println!("{:?}", table);

    for i in 0..16 {
      match i {
        6 | 7 | 8 | 9 | 10 => assert!(table.table[i] == vec![0]),
        _ => assert!(table.table[i].is_empty())
      }
    }
  }

  #[test]
  fn test_insert_line_6() {
    let vp = &Viewport::new(vec2![0., 0.], vec2![2., 2.], vec2![80., 80.]); // 田
    let mut table : SpatialHashTable<i32> = SpatialHashTable::default();
    table.init_viewport(vp);

    let l = Line { origin: vec2![0.0, -0.5], direction: vec2![(2.0 as f64).sqrt(), (2.0 as f64).sqrt()] };
    table.insert_line(0, l, vp);

    println!("{:?}", table);

    assert!(table.table[0].is_empty());
    assert!(table.table[1] == vec![0]);
    assert!(table.table[2] == vec![0]);
    assert!(table.table[3] == vec![0]);
  }
}