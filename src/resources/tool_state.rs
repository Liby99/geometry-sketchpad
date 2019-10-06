#[derive(Debug, Clone, Copy)]
pub enum Tool {
  Select,
  Point,
  Line,
  Circle,
  ViewportDrag,
}

impl Tool {
  pub fn depend_on_active_point(&self) -> bool {
    match self {
      Tool::Point | Tool::Line | Tool::Circle => true,
      _ => false,
    }
  }
}

pub struct ToolState(pub Tool);

impl Default for ToolState {
  fn default() -> Self {
    Self(Tool::Select)
  }
}

impl ToolState {
  pub fn depend_on_active_point(&self) -> bool {
    self.0.depend_on_active_point()
  }

  pub fn set(&mut self, tool: Tool) {
    self.0 = tool;
  }

  pub fn get(&self) -> Tool {
    self.0
  }
}