use crate::{events::*, resources::*};
use specs::prelude::*;

pub struct ViewportEventHandler {
    viewport_event_reader: Option<ViewportEventReader>,
}

impl Default for ViewportEventHandler {
    fn default() -> Self {
        Self {
            viewport_event_reader: None,
        }
    }
}

impl<'a> System<'a> for ViewportEventHandler {
    type SystemData = (Read<'a, ViewportEventChannel>, Write<'a, Viewport>);

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.viewport_event_reader =
            Some(world.fetch_mut::<ViewportEventChannel>().register_reader());
    }

    fn run(&mut self, (viewport_event_channel, mut viewport): Self::SystemData) {
        if let Some(reader) = &mut self.viewport_event_reader {
            for event in viewport_event_channel.read(reader) {
                match event {
                    ViewportEvent::Move(movement) => {
                        viewport.virtual_center = viewport.virtual_center + *movement;
                    }
                    ViewportEvent::Scale(rel_diff) => {
                        let new_x = viewport.virtual_size.x + rel_diff;
                        viewport.set_virtual_size_x(new_x);
                    }
                    ViewportEvent::Resize(scrn_size) => {
                        viewport.set_screen_size(*scrn_size);
                    }
                }
            }
        }
    }
}
