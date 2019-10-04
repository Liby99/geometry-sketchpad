use std::time::{SystemTime, Duration};

pub struct DeltaTime {
  last_recorded: SystemTime,
}

impl Default for DeltaTime {
  fn default() -> Self {
    Self { last_recorded: SystemTime::now() }
  }
}

impl DeltaTime {
  pub fn update(&mut self) {
    self.last_recorded = SystemTime::now();
  }

  pub fn duration(&self) -> Duration {
    self.last_recorded.elapsed().unwrap_or(Duration::from_millis(16))
  }
}