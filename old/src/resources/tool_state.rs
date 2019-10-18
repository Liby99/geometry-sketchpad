#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
  Select,
  Point,
  Line(LineTool),
  Circle,
  ViewportDrag,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineTool {
  Line,
  Ray,
  Segment,
}

impl Default for LineTool {
  fn default() -> Self {
    LineTool::Line
  }
}

impl Tool {
  pub fn depend_on_active_point(&self) -> bool {
    match self {
      Tool::Point | Tool::Line(_) | Tool::Circle => true,
      _ => false,
    }
  }

  pub fn default_line() -> Self {
    Tool::Line(LineTool::default())
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