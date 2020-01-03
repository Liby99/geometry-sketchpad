use crate::{components::markers::*, events::*};
use specs::prelude::*;

pub struct SelectHandler {
    command_event_reader: Option<CommandEventReader>,
}

impl Default for SelectHandler {
    fn default() -> Self {
        Self {
            command_event_reader: None,
        }
    }
}

impl<'a> System<'a> for SelectHandler {
    type SystemData = (
        Entities<'a>,
        Read<'a, CommandEventChannel>,
        Write<'a, MarkerEventChannel>,
        ReadStorage<'a, Element>,
        WriteStorage<'a, Selected>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.command_event_reader =
            Some(world.fetch_mut::<CommandEventChannel>().register_reader());
    }

    fn run(
        &mut self,
        (
    entities,
    command_event_channel,
    mut marker_event_channel,
    elements,
    mut selecteds
  ): Self::SystemData,
    ) {
        if let Some(reader) = &mut self.command_event_reader {
            for event in command_event_channel.read(reader) {
                match event {
                    CommandEvent::Select(select_event) => match select_event {
                        SelectEvent::Select(ent) => {
                            if let Err(err) = selecteds.insert(*ent, Selected) {
                                panic!(err)
                            }
                            marker_event_channel.single_write(MarkerEvent::Select(*ent));
                        }
                        SelectEvent::Deselect(ent) => {
                            selecteds.remove(*ent);
                            marker_event_channel.single_write(MarkerEvent::Deselect(*ent));
                        }
                        SelectEvent::SelectAll => {
                            for (ent, _) in (&entities, &elements).join() {
                                if let Err(err) = selecteds.insert(ent, Selected) {
                                    panic!(err)
                                }
                                marker_event_channel.single_write(MarkerEvent::Select(ent));
                            }
                        }
                        SelectEvent::DeselectAll => {
                            for (ent, _) in (&entities, &selecteds).join() {
                                marker_event_channel.single_write(MarkerEvent::Deselect(ent));
                            }
                            selecteds.clear();
                        }
                    },
                    _ => (),
                }
            }
        }
    }
}
