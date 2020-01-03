use crate::{events::*, resources::*, utilities::hitting_object};
use core_lib::{
    components::{markers::*, screen_shapes::*},
    events::*,
    math::*,
    resources::*,
    utilities::*,
};
use specs::prelude::*;
use std::collections::HashSet;
use std::mem::drop;

static SELECT_DIST_THRES: ScreenScalar = ScreenScalar(5.0); // Pixel

pub struct SeldeViaMouse {
    tool_change_reader: Option<ToolChangeEventReader>,
    mouse_event_reader: Option<MouseEventReader>,
    drag_start_position: Option<ScreenPosition>,
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
        Read<'a, SpatialEntityMap>,
        Write<'a, CommandEventChannel>,
        Write<'a, SelectRectangle>,
        ReadStorage<'a, ScreenPoint>,
        ReadStorage<'a, ScreenLine>,
        ReadStorage<'a, ScreenCircle>,
        ReadStorage<'a, Selected>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.tool_change_reader = Some(
            world
                .fetch_mut::<ToolChangeEventChannel>()
                .register_reader(),
        );
        self.mouse_event_reader = Some(world.fetch_mut::<MouseEventChannel>().register_reader());
    }

    fn run(
        &mut self,
        (
            input_state,
            tool_change_event_channel,
            mut mouse_event_channel,
            spatial_entity_map,
            mut command_event_channel,
            mut select_rectangle,
            scrn_points,
            scrn_lines,
            scrn_circles,
            selecteds,
        ): Self::SystemData,
    ) {
        // First use tool change to setup mouse event reader.
        // We will only listen to mouse event when the tool state is select.
        // We will drop the mouse event listener when the tool state is set to others.
        if let Some(reader_id) = &mut self.tool_change_reader {
            for event in tool_change_event_channel.read(reader_id) {
                match event {
                    ToolChangeEvent(Tool::Select) => {
                        self.mouse_event_reader = Some(mouse_event_channel.register_reader());
                    }
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
                        if let Some(entity) = hitting_object(
                            *mouse_pos,
                            &*spatial_entity_map,
                            &scrn_points,
                            &scrn_lines,
                            &scrn_circles,
                            SELECT_DIST_THRES,
                        ) {
                            // Check if shift is held
                            if input_state.keyboard.is_shift_activated() {
                                // If has shift, select or deselect based on previous state
                                if let Some(_) = selecteds.get(entity) {
                                    command_event_channel.single_write(CommandEvent::Select(
                                        SelectEvent::Deselect(entity),
                                    ));
                                } else {
                                    command_event_channel.single_write(CommandEvent::Select(
                                        SelectEvent::Select(entity),
                                    ));
                                }
                            } else {
                                // If no shift, always select
                                command_event_channel
                                    .single_write(CommandEvent::Select(SelectEvent::DeselectAll));
                                command_event_channel.single_write(CommandEvent::Select(
                                    SelectEvent::Select(entity),
                                ));
                            }
                        } else {
                            // Deselect all if not hitting anything
                            command_event_channel
                                .single_write(CommandEvent::Select(SelectEvent::DeselectAll));
                        }
                    }
                    MouseEvent::DragBegin(start_position) => {
                        // We need the dragging begin from an empty space
                        if hitting_object(
                            *start_position,
                            &*spatial_entity_map,
                            &scrn_points,
                            &scrn_lines,
                            &scrn_circles,
                            SELECT_DIST_THRES,
                        )
                        .is_none()
                        {
                            // If ther's no shift, clear the selection
                            if !input_state.keyboard.is_shift_activated() {
                                command_event_channel
                                    .single_write(CommandEvent::Select(SelectEvent::DeselectAll));
                            }

                            // Setup the drag start position
                            self.drag_start_position = Some(*start_position);
                        }
                    }
                    MouseEvent::DragMove(_, curr_position) => {
                        // Make sure we have start position before we set the dragging
                        if let Some(start_position) = self.drag_start_position {
                            // Update the rectangle
                            let rect =
                                AABB::two_points(start_position.into(), (*curr_position).into());
                            select_rectangle.set(rect);

                            // Select all the elements intersecting with AABB
                            let mut new_entities = get_entities_in_aabb(
                                rect,
                                &*spatial_entity_map,
                                &scrn_points,
                                &scrn_lines,
                                &scrn_circles,
                            );
                            let mut to_remove = vec![];
                            for entity in &self.drag_selected_new_entities {
                                if !new_entities.contains(entity) {
                                    to_remove.push(entity.clone());
                                    command_event_channel.single_write(CommandEvent::Select(
                                        SelectEvent::Deselect(*entity),
                                    ));
                                } else {
                                    new_entities.remove(entity);
                                }
                            }
                            for entity in to_remove {
                                self.drag_selected_new_entities.remove(&entity);
                            }
                            for entity in new_entities {
                                self.drag_selected_new_entities.insert(entity);
                                command_event_channel.single_write(CommandEvent::Select(
                                    SelectEvent::Select(entity),
                                ));
                            }
                        }
                    }
                    MouseEvent::DragEnd(_) => {
                        self.drag_start_position = None;
                        self.drag_selected_new_entities.clear();
                        select_rectangle.clear();
                    }
                    _ => (),
                }
            }
        }
    }
}

fn get_entities_in_aabb<'a>(
    aabb: AABB,
    spatial_entity_map: &SpatialEntityMap,
    scrn_points: &ReadStorage<'a, ScreenPoint>,
    scrn_lines: &ReadStorage<'a, ScreenLine>,
    scrn_circles: &ReadStorage<'a, ScreenCircle>,
) -> HashSet<Entity> {
    let mut result = HashSet::new();

    // Loop through all potential neighbors
    for entity in spatial_entity_map.get_entities_near_aabb(aabb) {
        if let Some(point) = scrn_points.get(entity) {
            if aabb.contains((*point).into()) {
                result.insert(entity);
            }
        } else if let Some(line) = scrn_lines.get(entity) {
            let line: Line = (*line).into();
            if line.intersect(aabb).is_some() {
                result.insert(entity);
            }
        } else if let Some(circle) = scrn_circles.get(entity) {
            let circle: Circle = (*circle).into();
            if circle.intersect(aabb).is_some() {
                result.insert(entity);
            }
        }
    }

    result
}
