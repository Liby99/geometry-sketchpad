use std::collections::HashSet;
use std::hash::Hash;
use crate::{math::*, resources::WINDOW_SIZE};

static TILE_SIZE : f64 = 40.0;

#[derive(Debug)]
pub struct SpatialHashTable<T: Clone + Eq + Hash> {
  width: f64,
  height: f64,
  x_tiles: usize,
  y_tiles: usize,
  table: Vec<HashSet<T>>,
}

impl<T: Clone + Eq + Hash> Default for SpatialHashTable<T> {
  fn default() -> Self {
    Self::new(WINDOW_SIZE[0], WINDOW_SIZE[1])
  }
}

type Tile = (i64, i64);

type TileId = usize;

impl<T: Clone + Eq + Hash> SpatialHashTable<T> {
  pub fn new(width: f64, height: f64) -> Self {
    let x_tiles = (width / TILE_SIZE).ceil() as usize;
    let y_tiles = (height / TILE_SIZE).ceil() as usize;
    Self {
      width,
      height,
      x_tiles,
      y_tiles,
      table: vec![HashSet::new(); x_tiles * y_tiles],
    }
  }

  pub fn set_size(&mut self, width: f64, height: f64) {
    self.width = width;
    self.height = height;
    self.x_tiles = (width / TILE_SIZE).ceil() as usize;
    self.y_tiles = (height / TILE_SIZE).ceil() as usize;
    self.table = vec![HashSet::new(); self.x_tiles * self.y_tiles];
  }

  pub fn clear(&mut self) {
    for set in &mut self.table {
      set.clear();
    }
  }

  fn tile_to_id(&self, tile: Tile) -> Option<TileId> {
    if 0 <= tile.0 && tile.0 < self.x_tiles as i64 && 0 <= tile.1 && tile.1 < self.y_tiles as i64 {
      Some(tile.1 as usize * self.x_tiles + tile.0 as usize)
    } else {
      None
    }
  }

  fn aabb(&self) -> AABB {
    AABB {
      x: 0.0,
      y: 0.0,
      width: self.width,
      height: self.height,
    }
  }

  fn tile_to_aabb(&self, tile: Tile) -> AABB {
    AABB {
      x: tile.0 as f64 * TILE_SIZE,
      y: tile.1 as f64 * TILE_SIZE,
      width: (tile.0 + 1) as f64 * TILE_SIZE,
      height: (tile.1 + 1) as f64 * TILE_SIZE,
    }
  }

  fn insert(&mut self, ent: T, tile: Tile) {
    if let Some(tile_id) = self.tile_to_id(tile) {
      self.table[tile_id].insert(ent);
    }
  }

  pub fn insert_point(&mut self, ent: T, p: Vector2) {
    self.insert(ent, self.get_tile(p))
  }

  pub fn insert_line(&mut self, ent: T, l: Line) {
    let aabb = self.aabb();
    if let Some((p1, p2)) = l.intersect(aabb) {

      // Make sure p1 is on the left and is not right at position 0
      let (p1, p2) = if p1.x > p2.x { (p2, p1) } else { (p1, p2) };
      let dir = (p2 - p1).normalized();
      let p1 = p1 + dir * 0.000001;

      // Get the tiles
      let (init_x_tile, init_y_tile) = self.get_tile(p1);
      let (end_x_tile, end_y_tile) = self.get_tile(p2);

      // Check if close to a straight line going up and down
      if init_x_tile == end_x_tile && init_x_tile >= 0 && init_x_tile < self.x_tiles as i64 {
        let (init_y_tile, end_y_tile) = if init_y_tile <= end_y_tile {
          (init_y_tile, end_y_tile)
        } else {
          (end_y_tile, init_y_tile)
        };
        for j in init_y_tile..(end_y_tile + 1) {
          self.insert(ent.clone(), (init_x_tile, j));
        }
      } else {

        // Setupt the state
        let yi = if dir.y < 0.0 { -1.0 } else { 1.0 };
        let mut curr_x = p1.x;
        let mut curr_y = p1.y;
        let mut curr_x_tile = init_x_tile as i64;
        let mut curr_y_tile = init_y_tile as i64;

        // Go through all the x tile in the same row that are covered by the line
        while curr_x_tile <= end_x_tile as i64 && 0 <= curr_y_tile && curr_y_tile < self.y_tiles as i64 {
          let next_y = (curr_y_tile + if dir.y > 0.0 { 1 } else { 0 }) as f64 * TILE_SIZE;
          let tile_offset_y = (next_y - curr_y) * yi;
          let next_x_diff = tile_offset_y / dir.y.abs() * dir.x;
          let next_x = curr_x + next_x_diff;
          let next_x_tile = (next_x / TILE_SIZE) as i64;
          for i in curr_x_tile..(next_x_tile + 1) {
            self.insert(ent.clone(), (i, curr_y_tile));
          }
          curr_x = next_x;
          curr_y = next_y;
          curr_x_tile = next_x_tile;
          curr_y_tile = curr_y_tile + yi as i64;
        }
      }
    }
  }

  pub fn insert_circle(&mut self, ent: T, c: Circle) {
    let (left, top) = self.get_tile(vec2![c.center.x - c.radius, c.center.y - c.radius]);
    let (right, bottom) = self.get_tile(vec2![c.center.x + c.radius, c.center.y + c.radius]);
    for j in top.max(0)..(bottom.min(self.x_tiles as i64) + 1) {
      for i in left.max(0)..(right.min(self.y_tiles as i64) + 1) {
        if 0 <= i && i < self.x_tiles as i64 && 0 <= j && j < self.y_tiles as i64 {
          let tile_aabb = self.tile_to_aabb((i, j));
          let closest_dist = (tile_aabb.get_closest_point_to(c.center) - c.center).magnitude();
          let furthest_dist = (tile_aabb.get_furthest_point_to(c.center) - c.center).magnitude();
          if closest_dist <= c.radius && closest_dist <= furthest_dist {
            self.insert(ent.clone(), (i, j));
          }
        }
      }
    }
  }

  pub fn remove_from_all(&mut self, ent: T) {
    for set in &mut self.table {
      set.remove(&ent);
    }
  }

  fn get_tile(&self, Vector2 { x, y }: Vector2) -> Tile {
    ((x / TILE_SIZE) as i64, (y / TILE_SIZE) as i64)
  }

  fn get_entities_in_tile(&self, tile: Tile) -> Option<&HashSet<T>> {
    if let Some(tile_id) = self.tile_to_id(tile) {
      Some(&self.table[tile_id])
    } else {
      None
    }
  }

  pub fn get_entities_near_aabb(&self, aabb: AABB) -> HashSet<T> {
    let mut entities = HashSet::new();
    if let Some(itsct) = self.aabb().intersect(aabb) {
      let (i_min, j_min) = self.get_tile(itsct.min());
      let (i_max, j_max) = self.get_tile(itsct.max());
      for j in j_min..(j_max + 1) {
        for i in i_min..(i_max + 1) {
          if let Some(tile_ents) = self.get_entities_in_tile((i, j)) {
            for ent in tile_ents {
              entities.insert(ent.clone());
            }
          }
        }
      }
    }
    return entities;
  }

  pub fn get_entities_near_point(&self, p: Vector2, dist: f64) -> HashSet<T> {
    let mut entities = HashSet::new();
    let (center_i, center_j) = self.get_tile(p);
    for j in (center_j - 1)..(center_j + 2) {
      for i in (center_i - 1)..(center_i + 2) {
        let tile_aabb = self.tile_to_aabb((i, j));
        let d = (tile_aabb.get_closest_point_to(p) - p).magnitude();
        if d <= dist {
          if let Some(tile_ents) = self.get_entities_in_tile((i, j)) {
            for ent in tile_ents {
              entities.insert(ent.clone());
            }
          }
        }
      }
    }
    return entities;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_sht_tile() {
    let sht = SpatialHashTable::<bool>::new(80., 80.);

    // Make sure there are only 4 (2x2) tiles in the sht
    assert!(sht.table.len() == 4);

    // Test the getting the correct tile
    assert!(sht.get_tile(vec2![0., 0.]) == (0, 0));
    assert!(sht.get_tile(vec2![20., 20.]) == (0, 0));
    assert!(sht.get_tile(vec2![50., 20.]) == (1, 0));
    assert!(sht.get_tile(vec2![50., 50.]) == (1, 1));

    // Test edge cases of tiling
    assert!(sht.get_tile(vec2![80., 80.]) == (2, 2));

    // Test getting tile id
    assert!(sht.tile_to_id(sht.get_tile(vec2![80., 80.])).is_none());
  }
}