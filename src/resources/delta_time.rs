use std::time::Duration;

pub struct DeltaTime(pub Duration);

impl Default for DeltaTime {
  fn default() -> Self {
    Self(Duration::from_millis(16))
  }
}