use std::collections::{HashMap, HashSet};
use specs::prelude::*;
use crate::{
  events::*,
  utilities::*,
  resources::*,
  components::{
    symbolics::*,
    styles::*,
    virtual_shapes::*,
    screen_shapes::*,
    markers::*,
  },
};

pub struct HistoryEventHandler {
  history_event_reader: Option<HistoryEventReader>,
}

impl Default for HistoryEventHandler {
  fn default() -> Self {
    Self { history_event_reader: None }
  }
}

impl<'a> System<'a> for HistoryEventHandler {
  type SystemData = (
    Read<'a, HistoryEventChannel>,
    Write<'a, GeometryEventChannel>,
    Write<'a, MarkerEventChannel>,
    Write<'a, History>,

    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, VirtualPoint>,
    WriteStorage<'a, ScreenPoint>,

    WriteStorage<'a, SymbolicLine>,
    WriteStorage<'a, LineStyle>,
    WriteStorage<'a, VirtualLine>,
    WriteStorage<'a, ScreenLine>,

    WriteStorage<'a, SymbolicCircle>,
    WriteStorage<'a, CircleStyle>,
    WriteStorage<'a, VirtualCircle>,
    WriteStorage<'a, ScreenCircle>,

    WriteStorage<'a, Element>,
    WriteStorage<'a, Selected>,
    WriteStorage<'a, Hidden>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.history_event_reader = Some(world.fetch_mut::<HistoryEventChannel>().register_reader());
  }

  fn run(&mut self, (
    history_event_channel,
    mut geometry_event_channel,
    mut marker_event_channel,
    mut history,

    mut sym_points,
    mut point_styles,
    mut virt_points,
    mut scrn_points,

    mut sym_lines,
    mut line_styles,
    mut virt_lines,
    mut scrn_lines,

    mut sym_circles,
    mut circle_styles,
    mut virt_circles,
    mut scrn_circles,

    mut elements,
    mut selecteds,
    mut hiddens,
  ): Self::SystemData) {
    if let Some(reader) = &mut self.history_event_reader {
      for event in history_event_channel.read(reader) {
        match event {
          HistoryEvent::Clear => {
            history.clear();
          },
          HistoryEvent::Undo => {
            if let Some(modification) = history.undo() {
              match modification {
                Modification::InsertMany(insertions) => {
                  write_remove_events(&mut geometry_event_channel, insertions);
                  for (ent, geom) in insertions {
                    remove_element(ent, geom, &mut sym_points, &mut point_styles, &mut virt_points, &mut scrn_points, &mut sym_lines, &mut line_styles, &mut virt_lines, &mut scrn_lines, &mut sym_circles, &mut circle_styles, &mut virt_circles, &mut scrn_circles, &mut elements, &mut selecteds, &mut hiddens);
                  }
                },
                Modification::RemoveMany(removals) => {
                  write_insert_events(&mut geometry_event_channel, removals);
                  for (ent, geom) in removals {
                    insert_element(ent, geom, &mut sym_points, &mut point_styles, &mut sym_lines, &mut line_styles, &mut sym_circles, &mut circle_styles, &mut elements, &mut selecteds);
                  }
                },
                Modification::Update(ent, old_geom, new_geom) => {
                  write_update_event(&mut geometry_event_channel, ent, new_geom, old_geom);
                  update_element(ent, old_geom, &mut sym_points, &mut point_styles, &mut sym_lines, &mut line_styles, &mut sym_circles, &mut circle_styles, &mut selecteds);
                },
                Modification::HideMany(unhidden_ents) => {
                  write_unhide_events(&mut marker_event_channel, unhidden_ents);
                  for ent in unhidden_ents {
                    unhide_element(ent, &mut hiddens);
                  }
                },
                Modification::UnhideMany(hidden_ents) => {
                  write_hide_events(&mut marker_event_channel, hidden_ents);
                  for ent in hidden_ents {
                    hide_element(ent, &mut hiddens);
                  }
                },
              }
            }
          },
          HistoryEvent::Redo => {
            if let Some(modification) = history.redo() {
              match modification {
                Modification::InsertMany(insertions) => {
                  write_insert_events(&mut geometry_event_channel, insertions);
                  for (ent, geom) in insertions {
                    insert_element(ent, geom, &mut sym_points, &mut point_styles, &mut sym_lines, &mut line_styles, &mut sym_circles, &mut circle_styles, &mut elements, &mut selecteds);
                  }
                },
                Modification::RemoveMany(removals) => {
                  write_remove_events(&mut geometry_event_channel, removals);
                  for (ent, geom) in removals {
                    remove_element(ent, geom, &mut sym_points, &mut point_styles, &mut virt_points, &mut scrn_points, &mut sym_lines, &mut line_styles, &mut virt_lines, &mut scrn_lines, &mut sym_circles, &mut circle_styles, &mut virt_circles, &mut scrn_circles, &mut elements, &mut selecteds, &mut hiddens);
                  }
                },
                Modification::Update(ent, old_geom, new_geom) => {
                  write_update_event(&mut geometry_event_channel, ent, old_geom, new_geom);
                  update_element(ent, new_geom, &mut sym_points, &mut point_styles, &mut sym_lines, &mut line_styles, &mut sym_circles, &mut circle_styles, &mut selecteds);
                },
                Modification::HideMany(unhidden_ents) => {
                  write_hide_events(&mut marker_event_channel, unhidden_ents);
                  for ent in unhidden_ents {
                    hide_element(ent, &mut hiddens);
                  }
                },
                Modification::UnhideMany(hidden_ents) => {
                  write_unhide_events(&mut marker_event_channel, hidden_ents);
                  for ent in hidden_ents {
                    unhide_element(ent, &mut hiddens);
                  }
                },
              }
            }
          },
        }
      }
    }
  }
}

fn remove_element<'a>(
  ent: &Entity,
  geom: &Geometry,

  sym_points: &mut WriteStorage<'a, SymbolicPoint>,
  point_styles: &mut WriteStorage<'a, PointStyle>,
  virt_points: &mut WriteStorage<'a, VirtualPoint>,
  scrn_points: &mut WriteStorage<'a, ScreenPoint>,

  sym_lines: &mut WriteStorage<'a, SymbolicLine>,
  line_styles: &mut WriteStorage<'a, LineStyle>,
  virt_lines: &mut WriteStorage<'a, VirtualLine>,
  scrn_lines: &mut WriteStorage<'a, ScreenLine>,

  sym_circles: &mut WriteStorage<'a, SymbolicCircle>,
  circle_styles: &mut WriteStorage<'a, CircleStyle>,
  virt_circles: &mut WriteStorage<'a, VirtualCircle>,
  scrn_circles: &mut WriteStorage<'a, ScreenCircle>,

  elements: &mut WriteStorage<'a, Element>,
  selecteds: &mut WriteStorage<'a, Selected>,
  hiddens: &mut WriteStorage<'a, Hidden>,
) {

  // Remove from markers
  elements.remove(*ent);
  selecteds.remove(*ent);
  hiddens.remove(*ent);

  // Remove from respective storages
  match geom {
    Geometry::Point(_, _) => {
      sym_points.remove(*ent);
      point_styles.remove(*ent);
      virt_points.remove(*ent);
      scrn_points.remove(*ent);
    },
    Geometry::Line(_, _) => {
      sym_lines.remove(*ent);
      line_styles.remove(*ent);
      virt_lines.remove(*ent);
      scrn_lines.remove(*ent);
    },
    Geometry::Circle(_, _) => {
      sym_circles.remove(*ent);
      circle_styles.remove(*ent);
      virt_circles.remove(*ent);
      scrn_circles.remove(*ent);
    }
  }
}

fn insert_element<'a>(
  ent: &Entity,
  geom: &Geometry,

  sym_points: &mut WriteStorage<'a, SymbolicPoint>,
  point_styles: &mut WriteStorage<'a, PointStyle>,

  sym_lines: &mut WriteStorage<'a, SymbolicLine>,
  line_styles: &mut WriteStorage<'a, LineStyle>,

  sym_circles: &mut WriteStorage<'a, SymbolicCircle>,
  circle_styles: &mut WriteStorage<'a, CircleStyle>,

  elements: &mut WriteStorage<'a, Element>,
  selecteds: &mut WriteStorage<'a, Selected>,
) {

  // Insert to markers
  if let Err(err) = elements.insert(*ent, Element) { panic!(err) }
  if let Err(err) = selecteds.insert(*ent, Selected) { panic!(err) }

  // Insert to respective storages
  match geom {
    Geometry::Point(sym_point, point_style) => {
      if let Err(err) = sym_points.insert(*ent, *sym_point) { panic!(err) }
      if let Err(err) = point_styles.insert(*ent, *point_style) { panic!(err) }
    },
    Geometry::Line(sym_line, line_style) => {
      if let Err(err) = sym_lines.insert(*ent, *sym_line) { panic!(err) }
      if let Err(err) = line_styles.insert(*ent, *line_style) { panic!(err) }
    },
    Geometry::Circle(sym_circle, circle_style) => {
      if let Err(err) = sym_circles.insert(*ent, *sym_circle) { panic!(err) }
      if let Err(err) = circle_styles.insert(*ent, *circle_style) { panic!(err) }
    },
  }
}

fn update_element<'a>(
  ent: &Entity,
  new_geom: &Geometry,

  sym_points: &mut WriteStorage<'a, SymbolicPoint>,
  point_styles: &mut WriteStorage<'a, PointStyle>,

  sym_lines: &mut WriteStorage<'a, SymbolicLine>,
  line_styles: &mut WriteStorage<'a, LineStyle>,

  sym_circles: &mut WriteStorage<'a, SymbolicCircle>,
  circle_styles: &mut WriteStorage<'a, CircleStyle>,

  selecteds: &mut WriteStorage<'a, Selected>,
) {

  // Select when inserted
  if let Err(err) = selecteds.insert(*ent, Selected) { panic!(err) }

  // Update respective storages
  match new_geom {
    Geometry::Point(sym_point, point_style) => {
      if let Err(err) = sym_points.insert(*ent, *sym_point) { panic!(err) }
      if let Err(err) = point_styles.insert(*ent, *point_style) { panic!(err) }
    },
    Geometry::Line(sym_line, line_style) => {
      if let Err(err) = sym_lines.insert(*ent, *sym_line) { panic!(err) }
      if let Err(err) = line_styles.insert(*ent, *line_style) { panic!(err) }
    },
    Geometry::Circle(sym_circle, circle_style) => {
      if let Err(err) = sym_circles.insert(*ent, *sym_circle) { panic!(err) }
      if let Err(err) = circle_styles.insert(*ent, *circle_style) { panic!(err) }
    },
  }
}

fn hide_element<'a>(ent: &Entity, hiddens: &mut WriteStorage<'a, Hidden>) {
  if let Err(err) = hiddens.insert(*ent, Hidden) { panic!(err) }
}

fn unhide_element<'a>(ent: &Entity, hiddens: &mut WriteStorage<'a, Hidden>) {
  hiddens.remove(*ent);
}

fn write_remove_events(geometry_event_channel: &mut GeometryEventChannel, entities: &HashMap<Entity, Geometry>) {
  for (entity, geometry) in entities {
    geometry_event_channel.single_write(GeometryEvent::removed_by_history(*entity, *geometry));
  }
}

fn write_insert_events(geometry_event_channel: &mut GeometryEventChannel, entities: &HashMap<Entity, Geometry>) {
  for (entity, geometry) in entities {
    geometry_event_channel.single_write(GeometryEvent::inserted_by_history(*entity, *geometry));
  }
}

fn write_update_event(geometry_event_channel: &mut GeometryEventChannel, ent: &Entity, old_geom: &Geometry, new_geom: &Geometry) {
  geometry_event_channel.single_write(GeometryEvent::updated_by_history(*ent, *old_geom, *new_geom));

  // We don't need this because the "update_finished" event is only used by history
  // geometry_event_channel.single_write(GeometryEvent::update_finished_by_history(*ent, *old_geom, *new_geom));
}

fn write_hide_events(marker_event_channel: &mut MarkerEventChannel, entities: &HashSet<Entity>) {
  for entity in entities {
    marker_event_channel.single_write(MarkerEvent::hide_by_history(*entity));
  }
}

fn write_unhide_events(marker_event_channel: &mut MarkerEventChannel, entities: &HashSet<Entity>) {
  for entity in entities {
    marker_event_channel.single_write(MarkerEvent::unhide_by_history(*entity));
  }
}