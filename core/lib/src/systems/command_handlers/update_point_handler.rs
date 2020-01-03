use crate::{components::symbolics::*, events::*};
use specs::prelude::*;

pub struct UpdatePointHandler {
    command_event_reader: Option<CommandEventReader>,
}

impl Default for UpdatePointHandler {
    fn default() -> Self {
        Self {
            command_event_reader: None,
        }
    }
}

impl<'a> System<'a> for UpdatePointHandler {
    type SystemData = (
        Read<'a, CommandEventChannel>,
        Write<'a, GeometryEventChannel>,
        WriteStorage<'a, SymbolicPoint>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.command_event_reader =
            Some(world.fetch_mut::<CommandEventChannel>().register_reader());
    }

    fn run(
        &mut self,
        (command_event_channel, mut geometry_event_channel, mut sym_points): Self::SystemData,
    ) {
        if let Some(reader) = &mut self.command_event_reader {
            for event in command_event_channel.read(reader) {
                match event {
                    CommandEvent::Update(update_event) => match update_event {
                        UpdateEvent::UpdatePoint(ent, old_sym_point, new_sym_point) => {
                            if let Err(err) = sym_points.insert(*ent, *new_sym_point) {
                                panic!(err)
                            }
                            geometry_event_channel.single_write(GeometryEvent::point_updated(
                                *ent,
                                *old_sym_point,
                                *new_sym_point,
                            ));
                        }
                        UpdateEvent::UpdatePointEnd(ent, old_sym_point, new_sym_point) => {
                            if let Err(err) = sym_points.insert(*ent, *new_sym_point) {
                                panic!(err)
                            }
                            geometry_event_channel.single_write(
                                GeometryEvent::point_update_finished(
                                    *ent,
                                    *old_sym_point,
                                    *new_sym_point,
                                ),
                            );
                        }
                        UpdateEvent::UpdatePointByHistory(ent, old_sym_point, new_sym_point) => {
                            if let Err(err) = sym_points.insert(*ent, *new_sym_point) {
                                panic!(err)
                            }
                            geometry_event_channel.single_write(
                                GeometryEvent::point_updated_by_history(
                                    *ent,
                                    *old_sym_point,
                                    *new_sym_point,
                                ),
                            );
                            geometry_event_channel.single_write(
                                GeometryEvent::point_update_finished_by_history(
                                    *ent,
                                    *old_sym_point,
                                    *new_sym_point,
                                ),
                            );
                        }
                    },
                    _ => (),
                }
            }
        }
    }
}
