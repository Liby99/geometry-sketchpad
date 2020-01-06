use crate::{events::*, resources::*, utilities::hitting_object};
use core_lib::{
  components::{screen_shapes::*, symbolics::*},
  events::*,
  math::*,
  resources::*,
  utilities::*,
};
use specs::prelude::*;

static SELECT_DIST_THRES: ScreenScalar = ScreenScalar(5.0); // Pixel

pub struct MovePointViaDrag {
  tool_change_event_reader: Option<ToolChangeEventReader>,
  mouse_event_reader: Option<MouseEventReader>,
  dragging_point: Option<(Entity, SymbolicPoint)>,
  start_position: Option<ScreenPoint>,
}

impl Default for MovePointViaDrag {
  fn default() -> Self {
    Self {
      tool_change_event_reader: None,
      mouse_event_reader: None,
      dragging_point: None,
      start_position: None,
    }
  }
}

impl<'a> System<'a> for MovePointViaDrag {
  type SystemData = (
    Read<'a, InputState>,
    Read<'a, ToolChangeEventChannel>,
    Write<'a, MouseEventChannel>,
    Read<'a, SpatialEntityMap>,
    Read<'a, Viewport>,
    Write<'a, CommandEventChannel>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, ScreenPoint>,
    ReadStorage<'a, ScreenLine>,
    ReadStorage<'a, ScreenCircle>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_event_reader = Some(world.fetch_mut::<ToolChangeEventChannel>().register_reader());
    self.mouse_event_reader = Some(world.fetch_mut::<MouseEventChannel>().register_reader());
  }

  fn run(
    &mut self,
    (
      input_state,
      tool_change_event_channel,
      mut mouse_event_channel,
      spatial_entity_map,
      viewport,
      mut command_event_channel,
      sym_points,
      scrn_points,
      scrn_lines,
      scrn_circles,
    ): Self::SystemData,
  ) {
    // First use tool change to setup mouse event reader.
    // We will only listen to mouse event when the tool state is select.
    // We will drop the mouse event listener when the tool state is set to others.
    if let Some(reader_id) = &mut self.tool_change_event_reader {
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

    if let Some(reader_id) = &mut self.mouse_event_reader {
      for event in mouse_event_channel.read(reader_id) {
        match event {
          MouseEvent::DragBegin(start_position) => {
            if !input_state.keyboard.is_shift_activated() {
              if let Some(entity) = hitting_object(
                *start_position,
                &spatial_entity_map,
                &scrn_points,
                &scrn_lines,
                &scrn_circles,
                SELECT_DIST_THRES,
              ) {
                if let Some(sym_point) = sym_points.get(entity) {
                  self.dragging_point = Some((entity, *sym_point));
                  self.start_position = Some(*start_position);

                  // Note that we let the dragging point to be selected directly
                  command_event_channel.single_write(CommandEvent {
                    command: Command::Select(SelectEvent::Select(entity)),
                    event_id: None,
                  });
                }
              }
            }
          }
          MouseEvent::DragMove(_, curr_position) => match self.dragging_point {
            Some((ent, _)) => {
              if let Some(old_sym_point) = sym_points.get(ent) {
                if let Some(new_sym_point) =
                  get_update(*old_sym_point, *curr_position, &viewport, &scrn_lines, &scrn_circles)
                {
                  command_event_channel.single_write(CommandEvent {
                    command: Command::Update(UpdateEvent::UpdatePoint(ent, *old_sym_point, new_sym_point)),
                    event_id: None,
                  });
                }
              }
            }
            None => (),
          },
          MouseEvent::DragEnd(curr_position) => {
            match self.dragging_point {
              Some((ent, old_sym_point)) => {
                if let Some(new_sym_point) =
                  get_update(old_sym_point, *curr_position, &viewport, &scrn_lines, &scrn_circles)
                {
                  command_event_channel.single_write(CommandEvent {
                    command: Command::Update(UpdateEvent::UpdatePointEnd(ent, old_sym_point, new_sym_point)),
                    event_id: None,
                  });
                }
              }
              None => (),
            }
            self.dragging_point = None;
          }
          _ => (),
        }
      }
    }
  }
}

fn get_update<'a>(
  old_sym_point: SymbolicPoint,
  curr_position: ScreenPosition,
  viewport: &Viewport,
  scrn_lines: &ReadStorage<'a, ScreenLine>,
  scrn_circles: &ReadStorage<'a, ScreenCircle>,
) -> Option<SymbolicPoint> {
  match old_sym_point {
    SymbolicPoint::Free(_) => {
      let new_position = curr_position.to_virtual(&viewport);
      Some(SymbolicPoint::Free(new_position))
    }
    SymbolicPoint::OnLine(l_ent, _) => {
      if let Some(line) = scrn_lines.get(l_ent) {
        let closest_point = line.get_closest_point(curr_position);
        let new_t = line.rel_t_of_point(closest_point);
        Some(SymbolicPoint::OnLine(l_ent, new_t.into()))
      } else {
        None
      }
    }
    SymbolicPoint::OnCircle(c_ent, _) => {
      if let Some(circle) = scrn_circles.get(c_ent) {
        let projected_position = curr_position.project(*circle);
        let p_to_cen: Vector2 = (projected_position - circle.center).into();
        let new_theta = -p_to_cen.y.atan2(p_to_cen.x);
        Some(SymbolicPoint::OnCircle(c_ent, new_theta))
      } else {
        None
      }
    }
    _ => None,
  }
}
