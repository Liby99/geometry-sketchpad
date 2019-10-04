pub enum ToolState {
  Select,
  Point,
  Line,
  Circle,
}

impl Default for ToolState {
  fn default() -> Self {
    ToolState::Select
  }
}

impl ToolState {
  pub fn depend_on_active_point(&self) -> bool {
    match self {
      ToolState::Point | ToolState::Line | ToolState::Circle => true,
      _ => false,
    }
  }
}