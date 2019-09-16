pub enum ToolState {
  Select,
  Point,
}

impl Default for ToolState {
  fn default() -> Self {
    Self::Select
  }
}