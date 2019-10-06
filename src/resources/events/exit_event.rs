use shrev::{EventChannel, ReaderId};

pub struct ExitEvent;

pub type ExitEventChannel = EventChannel<ExitEvent>;

pub type ExitEventReader = ReaderId<ExitEvent>;