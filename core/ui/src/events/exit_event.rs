use shrev::*;

pub struct ExitEvent;

pub type ExitEventReader = ReaderId<ExitEvent>;

pub type ExitEventChannel = EventChannel<ExitEvent>;