use specs::prelude::*;
use shrev::*;

pub struct ActivePointEvent(pub Entity);

pub type ActivePointEventChannel = EventChannel<ActivePointEvent>;

pub type ActivePointEventReader = ReaderId<ActivePointEvent>;