use specs::prelude::*;
use super::output::RenderUpdateEvent;

pub struct SenderSystem {
  pub sender: std::sync::mpsc::Sender<RenderUpdateEvent>,
}

impl<'a> System<'a> for SenderSystem {
  type SystemData = (

  );

  fn run(&mut self, (): Self::SystemData) {
    if let Err(err) = self.sender.send(RenderUpdateEvent::None) { panic!(err) };
  }
}