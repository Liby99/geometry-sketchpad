use core_lib::math::AABB;

pub struct SelectRectangle(pub Option<AABB>);

impl Default for SelectRectangle {
  fn default() -> Self {
    Self(None)
  }
}

impl SelectRectangle {
  pub fn set(&mut self, aabb: AABB) {
    self.0 = Some(aabb);
  }

  pub fn clear(&mut self) {
    self.0 = None;
  }

  pub fn get(&self) -> Option<AABB> {
    self.0
  }
}
