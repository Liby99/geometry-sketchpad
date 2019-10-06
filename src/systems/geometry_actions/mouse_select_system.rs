use std::mem::drop;
use specs::prelude::*;
use crate::{
  util::Vector2,
  resources::{Viewport, ViewportTransform, SpatialHashTable, InputState, Tool},
  components::{Point, Line, Selected},
  systems::events::{
    MouseEvent, MouseEventChannel, MouseEventReader,
    ToolChangeEvent, ToolChangeEventChannel, ToolChangeEventReader,
    SketchEventChannel, SketchEvent,
    GeometryActionChannel, GeometryAction,
  },
};

static SELECT_DIST_THRES : f64 = 5.0; // Pixel

pub struct MouseSelectSystem {
  tool_change_reader: Option<ToolChangeEventReader>,
  mouse_event_reader: Option<MouseEventReader>,
  // drag_start_position: Option<Vector2>,
}

impl Default for MouseSelectSystem {
  fn default() -> Self {
    Self {
      tool_change_reader: None,
      mouse_event_reader: None,
      // drag_start_position: None,
    }
  }
}

impl<'a> System<'a> for MouseSelectSystem {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, ToolChangeEventChannel>,
    Write<'a, MouseEventChannel>,
    Read<'a, Viewport>,
    Read<'a, SpatialHashTable<Entity>>,
    Write<'a, GeometryActionChannel>,
    Write<'a, SketchEventChannel>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_reader = Some(world.fetch_mut::<ToolChangeEventChannel>().register_reader());
  }

  fn run(&mut self, (
    input_state,
    tool_change_event_channel,
    mut mouse_event_channel,
    viewport,
    spatial_table,
    mut geometry_action_channel,
    mut sketch_event_channel,
    points,
    lines,
    selected,
  ): Self::SystemData) {

    // First use tool change to setup mouse event reader.
    // We will only listen to mouse event when the tool state is select.
    // We will drop the mouse event listener when the tool state is set to others.
    if let Some(reader_id) = &mut self.tool_change_reader {
      for event in tool_change_event_channel.read(reader_id) {
        match event {
          ToolChangeEvent(Tool::Select) => {
            self.mouse_event_reader = Some(mouse_event_channel.register_reader());
          },
          _ => {
            if let Some(reader_id) = &mut self.mouse_event_reader {
              drop(reader_id);
              self.mouse_event_reader = None;
            }
          }
        }
      }
    }

    // Read the mouse event
    if let Some(reader_id) = &mut self.mouse_event_reader {
      for event in mouse_event_channel.read(reader_id) {
        match event {
          MouseEvent::MouseDown(mouse_pos) => {

            // If there's no shift
            let mut already_deselected_all = false;
            if !input_state.keyboard.is_shift_activated() {
              already_deselected_all = true;
              geometry_action_channel.single_write(GeometryAction::DeselectAll);
            }

            // Check if hitting something
            if let Some(entity) = hitting_object(*mouse_pos, &*viewport, &*spatial_table, &points, &lines) {
              if let Some(_) = selected.get(entity) {
                sketch_event_channel.single_write(SketchEvent::Deselect(entity));
              } else {
                sketch_event_channel.single_write(SketchEvent::Select(entity));
              }
            } else {

              // If hit nothing, still deselect everything
              if !already_deselected_all {
                geometry_action_channel.single_write(GeometryAction::DeselectAll);
              }
            }
          },
          MouseEvent::DragBegin(_) => {

          },
          MouseEvent::DragMove(_) => {

          },
          MouseEvent::DragEnd(_) => {

          },
          _ => (),
        }
      }
    }
  }
}

fn hitting_object<'a>(
  mouse_pos: Vector2,
  viewport: &Viewport,
  spatial_table: &SpatialHashTable<Entity>,
  points: &ReadStorage<'a, Point>,
  lines: &ReadStorage<'a, Line>,
) -> Option<Entity> {

  // First get the virtual mouse position
  let virtual_mouse_pos = mouse_pos.to_virtual(viewport);

  // Use spatial hash table to get potential neighbors
  let maybe_neighbors = spatial_table.get_neighbor_entities(virtual_mouse_pos, viewport);
  let mut maybe_selected_point : Option<(Entity, f64)> = None;
  let mut maybe_selected_line : Option<(Entity, f64)> = None;
  if let Some(neighbor_entities) = maybe_neighbors {
    for entity in neighbor_entities {
      if let Some(p) = points.get(entity) {
        let dist = (p.to_actual(viewport) - mouse_pos).magnitude();
        if dist < SELECT_DIST_THRES && (maybe_selected_point.is_none() || dist < maybe_selected_point.unwrap().1) {
          maybe_selected_point = Some((entity, dist));
        }
      } else if let Some(l) = lines.get(entity) {
        let actual_proj_point = mouse_pos.project(l.to_actual(viewport));
        let dist = (actual_proj_point - mouse_pos).magnitude();
        if dist < SELECT_DIST_THRES && (maybe_selected_line.is_none() || dist < maybe_selected_line.unwrap().1) {
          maybe_selected_line = Some((entity, dist));
        }
      }
    }
  }

  // Return point in priority to line
  maybe_selected_point.or(maybe_selected_line).map(|(ent, _)| ent)
}