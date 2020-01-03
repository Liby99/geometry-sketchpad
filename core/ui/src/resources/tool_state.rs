use core_lib::math::*;

#[derive(Copy, Clone, Debug)]
pub enum Tool {
    Select,
    Viewport,
    Point,
    Line(LineType),
    Circle,
}

impl Tool {
    pub fn need_snap_point(&self) -> bool {
        match self {
            Tool::Point | Tool::Line(_) | Tool::Circle => true,
            _ => false,
        }
    }
}

pub struct ToolState(Tool);

impl Default for ToolState {
    fn default() -> Self {
        Self(Tool::Select)
    }
}

impl ToolState {
    pub fn get(&self) -> Tool {
        self.0
    }

    pub fn set(&mut self, tool: Tool) {
        self.0 = tool;
    }

    pub fn need_snap_point(&self) -> bool {
        self.0.need_snap_point()
    }
}
