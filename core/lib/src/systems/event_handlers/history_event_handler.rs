use crate::{components::symbolics::*, events::*, resources::*, utilities::*};
use specs::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct HistoryEventHandler {
  history_event_reader: Option<HistoryEventReader>,
}

impl Default for HistoryEventHandler {
  fn default() -> Self {
    Self {
      history_event_reader: None,
    }
  }
}

impl<'a> System<'a> for HistoryEventHandler {
  type SystemData = (
    Read<'a, HistoryEventChannel>,
    Write<'a, CommandEventChannel>,
    Write<'a, History>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.history_event_reader = Some(world.fetch_mut::<HistoryEventChannel>().register_reader());
  }

  fn run(&mut self, (history_event_channel, mut command_event_channel, mut history): Self::SystemData) {
    if let Some(reader) = &mut self.history_event_reader {
      for event in history_event_channel.read(reader) {
        match event {
          HistoryEvent::Clear => {
            history.clear();
          }
          HistoryEvent::Undo => {
            if let Some(modification) = history.undo() {
              match modification {
                Modification::InsertMany(insertions) => write_remove_events(&mut command_event_channel, insertions),
                Modification::RemoveMany(removals) => write_insert_events(&mut command_event_channel, removals),
                Modification::UpdatePoint(ent, old_sym_point, new_sym_point) => {
                  write_update_event(&mut command_event_channel, ent, new_sym_point, old_sym_point)
                }
                Modification::HideMany(unhidden_ents) => write_unhide_events(&mut command_event_channel, unhidden_ents),
                Modification::UnhideMany(hidden_ents) => write_hide_events(&mut command_event_channel, hidden_ents),
              }
            }
          }
          HistoryEvent::Redo => {
            if let Some(modification) = history.redo() {
              match modification {
                Modification::InsertMany(insertions) => write_insert_events(&mut command_event_channel, insertions),
                Modification::RemoveMany(removals) => write_remove_events(&mut command_event_channel, removals),
                Modification::UpdatePoint(ent, old_sym_point, new_sym_point) => {
                  write_update_event(&mut command_event_channel, ent, old_sym_point, new_sym_point)
                }
                Modification::HideMany(unhidden_ents) => write_hide_events(&mut command_event_channel, unhidden_ents),
                Modification::UnhideMany(hidden_ents) => write_unhide_events(&mut command_event_channel, hidden_ents),
              }
            }
          }
        }
      }
    }
  }
}

fn write_remove_events(command_event_channel: &mut CommandEventChannel, entities: &HashMap<Entity, Geometry>) {
  for (entity, _) in entities {
    command_event_channel.single_write(CommandEvent {
      command: Command::Remove(RemoveEvent::RemoveByHistory(*entity)),
      event_id: None,
    });
  }
}

fn write_insert_events(command_event_channel: &mut CommandEventChannel, entities: &HashMap<Entity, Geometry>) {
  for (ent, geometry) in entities {
    let command = match geometry {
      Geometry::Point(sym_point, point_style) => CommandEvent {
        command: Command::PointInsert(InsertPointEvent::InsertPointByHistory(*ent, *sym_point, *point_style)),
        event_id: None,
      },
      Geometry::Line(sym_line, line_style) => CommandEvent {
        command: Command::LineInsert(InsertLineEvent::InsertLineByHistory(*ent, *sym_line, *line_style)),
        event_id: None,
      },
      Geometry::Circle(sym_circle, circle_style) => CommandEvent {
        command: Command::CircleInsert(InsertCircleEvent::InsertCircleByHistory(
          *ent,
          *sym_circle,
          *circle_style,
        )),
        event_id: None,
      },
    };
    command_event_channel.single_write(command);
  }
}

fn write_update_event(
  command_event_channel: &mut CommandEventChannel,
  ent: &Entity,
  old_sym_point: &SymbolicPoint,
  new_sym_point: &SymbolicPoint,
) {
  // We don't need this line because the "update_finished" event is only used by history
  // geometry_event_channel.single_write(GeometryEvent::update_finished_by_history(*ent, *old_geom, *new_geom));
  command_event_channel.single_write(CommandEvent {
    command: Command::Update(UpdateEvent::UpdatePointByHistory(*ent, *old_sym_point, *new_sym_point)),
    event_id: None,
  });
}

fn write_hide_events(command_event_channel: &mut CommandEventChannel, entities: &HashSet<Entity>) {
  for entity in entities {
    command_event_channel.single_write(CommandEvent {
      command: Command::Hide(HideEvent::HideByHistory(*entity)),
      event_id: None,
    });
  }
}

fn write_unhide_events(command_event_channel: &mut CommandEventChannel, entities: &HashSet<Entity>) {
  for entity in entities {
    command_event_channel.single_write(CommandEvent {
      command: Command::Hide(HideEvent::UnhideByHistory(*entity)),
      event_id: None,
    });
  }
}
