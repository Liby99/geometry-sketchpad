use std::mem::drop;
use specs::prelude::*;
use crate::{
  util::{Vector2, Color},
  resources::{Viewport, ViewportTransform, SpatialHashTable, InputState, Tool},
  components::{Point, Line, LineStyle, Selected, Rectangle, RectangleStyle},
  systems::events::{
    MouseEvent, MouseEventChannel, MouseEventReader,
    ToolChangeEvent, ToolChangeEventChannel, ToolChangeEventReader,
    SketchEventChannel, SketchEvent,
    GeometryActionChannel, GeometryAction,
  },
};

static SELECT_DIST_THRES : f64 = 5.0; // Pixel
static SELECT_RECT_STYLE : RectangleStyle = RectangleStyle {
  border: LineStyle {
    color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.3 },
    width: 1.,
  },
  fill: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.05 },
};

pub struct MouseSelectSystem {
  tool_change_reader: Option<ToolChangeEventReader>,
  mouse_event_reader: Option<MouseEventReader>,
  drag_rectangle_entity: Option<Entity>,
  drag_start_position: Option<Vector2>,
}

impl Default for MouseSelectSystem {
  fn default() -> Self {
    Self {
      tool_change_reader: None,
      mouse_event_reader: None,
      drag_rectangle_entity: None,
      drag_start_position: None,
    }
  }
}

impl<'a> System<'a> for MouseSelectSystem {
  type SystemData = (
    Entities<'a>,
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
    WriteStorage<'a, Rectangle>,
    WriteStorage<'a, RectangleStyle>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_reader = Some(world.fetch_mut::<ToolChangeEventChannel>().register_reader());
    self.mouse_event_reader = Some(world.fetch_mut::<MouseEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
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
    mut rects,
    mut rect_styles,
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
          MouseEvent::DragBegin(start_position) => {

            // We need the dragging begin from an empty space
            if hitting_object(*start_position, &*viewport, &*spatial_table, &points, &lines).is_none() {

              // If ther's no shift, clear the selection
              if !input_state.keyboard.is_shift_activated() {
                geometry_action_channel.single_write(GeometryAction::DeselectAll);
              }

              // Setup the drag start position
              self.drag_start_position = Some(*start_position);
            }
          },
          MouseEvent::DragMove(_) => {

            // Make sure we have the rectangle entity
            let rect_ent = if let Some(ent) = self.drag_rectangle_entity { ent } else {
              let ent = entities.create();
              self.drag_rectangle_entity = Some(ent);
              if let Err(err) = rect_styles.insert(ent, SELECT_RECT_STYLE) { panic!(err) }
              ent
            };

            // Get the
            let start_position = self.drag_start_position.unwrap_or(vec2![0., 0.]);
            let curr_position = input_state.mouse_abs_pos;
            let diff = curr_position - start_position;

            // Update the rectangle
            if let Err(err) = rects.insert(rect_ent, Rectangle {
              x: start_position.x,
              y: start_position.y,
              width: diff.x,
              height: diff.y
            }) { panic!(err) }

            // TODO: select the things in the middle
          },
          MouseEvent::DragEnd(_) => {

            // Remove the rectangle information when dragging ends
            if let Some(ent) = self.drag_rectangle_entity {
              self.drag_start_position = None;
              rects.remove(ent);
            }
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