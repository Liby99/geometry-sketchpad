use crate::resources::Tool;
use shrev::*;

pub struct ToolChangeEvent(pub Tool);

pub type ToolChangeEventChannel = EventChannel<ToolChangeEvent>;

pub type ToolChangeEventReader = ReaderId<ToolChangeEvent>;
