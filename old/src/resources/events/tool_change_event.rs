use shrev::{EventChannel, ReaderId};
use crate::resources::Tool;

pub struct ToolChangeEvent(pub Tool);

pub type ToolChangeEventChannel = EventChannel<ToolChangeEvent>;

pub type ToolChangeEventReader = ReaderId<ToolChangeEvent>;