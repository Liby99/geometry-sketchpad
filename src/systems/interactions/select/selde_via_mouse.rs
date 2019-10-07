use std::mem::drop;
use std::collections::HashSet;
use specs::prelude::*;
use crate::{
  utilities::{Vector2, AABB, Intersect},
  resources::{
    Viewport, ViewportTransform,
    SpatialHashTable,
    InputState,
    Tool,
    geometry::SelectRectangle,
    events::{
      MouseEvent, MouseEventChannel, MouseEventReader,
      ToolChangeEvent, ToolChangeEventChannel, ToolChangeEventReader,
      SketchEventChannel, SketchEvent,
      GeometryActionChannel, GeometryAction,
    },
  },
  components::{Point, Line, Selected},
};
use super::super::helpers::hitting_object;

static SELECT_DIST_THRES : f64 = 5.0; // Pixel

pub struct SeldeViaMouse {
  tool_change_reader: Option<ToolChangeEventReader>,
  mouse_event_reader: Option<MouseEventReader>,
  drag_start_position: Option<Vector2>,
  drag_selected_new_entities: HashSet<Entity>,
}

impl Default for SeldeViaMouse {
  fn default() -> Self {
    Self {
      tool_change_reader: None,
      mouse_event_reader: None,
      drag_start_position: None,
      drag_selected_new_entities: HashSet::new(),
    }
  }
}

impl<'a> System<'a> for SeldeViaMouse {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, ToolChangeEventChannel>,
    Write<'a, MouseEventChannel>,
    Read<'a, Viewport>,
    Read<'a, SpatialHashTable<Entity>>,
    Write<'a, GeometryActionChannel>,
    Write<'a, SketchEventChannel>,
    Write<'a, SelectRectangle>,
    ReadStorage<'a, Point>,
    ReadStorage<'a, Line>,
    ReadStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_reader = Some(world.fetch_mut::<ToolChangeEventChannel>().register_reader());
    self.mouse_event_reader = Some(world.fetch_mut::<MouseEventChannel>().register_reader());
  }

  fn run(&mut self, (
    input_state,
    tool_change_event_channel,
    mut mouse_event_channel,
    viewport,
    spatial_table,
    mut geometry_action_channel,
    mut sketch_event_channel,
    mut select_rectangle,
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

            // Check if hitting something
            if let Some(entity) = hitting_object(*mouse_pos, &*viewport, &*spatial_table, &points, &lines, SELECT_DIST_THRES) {

              // Check if shift is held
              if input_state.keyboard.is_shift_activated() {

                // If has shift, select or deselect based on previous state
                if let Some(_) = selected.get(entity) {
                  sketch_event_channel.single_write(SketchEvent::Deselect(entity));
                } else {
                  sketch_event_channel.single_write(SketchEvent::Select(entity));
                }
              } else {

                // If no shift, always select
                geometry_action_channel.single_write(GeometryAction::DeselectAllExcept(entity));
                sketch_event_channel.single_write(SketchEvent::Select(entity));
              }
            } else {

              // Deselect all if not hitting anything
              geometry_action_channel.single_write(GeometryAction::DeselectAll);
            }
          },
          MouseEvent::DragBegin(start_position) => {

            // We need the dragging begin from an empty space
            if hitting_object(*start_position, &*viewport, &*spatial_table, &points, &lines, SELECT_DIST_THRES).is_none() {

              // If ther's no shift, clear the selection
              if !input_state.keyboard.is_shift_activated() {
                geometry_action_channel.single_write(GeometryAction::DeselectAll);
              }

              // Setup the drag start position
              self.drag_start_position = Some(*start_position);
            }
          },
          MouseEvent::DragMove(_, curr_position) => {

            // Make sure we have start position before we set the dragging
            if let Some(start_position) = self.drag_start_position {

              // Get the current to start difference
              let diff = *curr_position - start_position;

              // Update the rectangle
              let rect = AABB {
                x: start_position.x.min(curr_position.x),
                y: start_position.y.min(curr_position.y),
                width: diff.x.abs(),
                height: diff.y.abs(),
              };
              select_rectangle.set(rect);

              // Select all the elements intersecting with AABB
              let mut new_entities = get_entities_in_aabb(rect, &*viewport, &*spatial_table, &points, &lines);
              let mut to_remove = vec![];
              for entity in &self.drag_selected_new_entities {
                if !new_entities.contains(entity) {
                  to_remove.push(entity.clone());
                  sketch_event_channel.single_write(SketchEvent::Deselect(*entity));
                } else {
                  new_entities.remove(entity);
                }
              }
              for entity in to_remove {
                self.drag_selected_new_entities.remove(&entity);
              }
              for entity in new_entities {
                self.drag_selected_new_entities.insert(entity);
                sketch_event_channel.single_write(SketchEvent::Select(entity));
              }
            }
          },
          MouseEvent::DragEnd(_) => {
            self.drag_start_position = None;
            self.drag_selected_new_entities.clear();
            select_rectangle.clear();
          },
          _ => (),
        }
      }
    }
  }
}

fn get_entities_in_aabb<'a>(
  aabb: AABB,
  viewport: &Viewport,
  spatial_table: &SpatialHashTable<Entity>,
  points: &ReadStorage<'a, Point>,
  lines: &ReadStorage<'a, Line>,
) -> HashSet<Entity> {
  let mut result = HashSet::new();

  // Loop through all potential neighbors
  for entity in spatial_table.get_neighbor_entities_of_aabb(aabb) {
    if let Some(point) = points.get(entity) {
      let actual = point.to_actual(viewport);
      if aabb.contains(actual) {
        result.insert(entity);
      }
    } else if let Some(line) = lines.get(entity) {
      let actual = line.to_actual(viewport);
      if actual.intersect(aabb).is_some() {
        result.insert(entity);
      }
    }
  }

  result
}