use specs::prelude::*;
use geopad_core_lib::{math::*, events::*, resources::*};
use crate::{events::*, resources::*};

pub struct ViewportDragTool {
  tool_change_event_reader: Option<ToolChangeEventReader>,
  mouse_event_reader: Option<MouseEventReader>,
}

impl Default for ViewportDragTool {
  fn default() -> Self {
    Self {
      tool_change_event_reader: None,
      mouse_event_reader: None
    }
  }
}

impl<'a> System<'a> for ViewportDragTool {
  type SystemData = (
    // Read<'a, InputState>,
    Read<'a, Viewport>,
    Read<'a, ToolChangeEventChannel>,
    Write<'a, MouseEventChannel>,
    Write<'a, ViewportEventChannel>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_event_reader = Some(world.fetch_mut::<ToolChangeEventChannel>().register_reader());
  }

  fn run(&mut self, (
    // input_state,
    viewport,
    tool_change_event_channel,
    mut mouse_event_channel,
    mut viewport_event_channel,
  ): Self::SystemData) {

    // Register mouse event reader in regards to tool change
    if let Some(reader) = &mut self.tool_change_event_reader {
      for ToolChangeEvent(tool) in tool_change_event_channel.read(reader) {
        match tool {
          Tool::Viewport => {
            if self.mouse_event_reader.is_none() {
              self.mouse_event_reader = Some(mouse_event_channel.register_reader());
            }
          },
          _ => {
            if let Some(reader) = &mut self.mouse_event_reader {
              std::mem::drop(reader);
              self.mouse_event_reader = None;
            }
          }
        }
      }
    }

    // Read mouse events. If so then that means we are using viewport tool now
    if let Some(reader) = &mut self.mouse_event_reader {

      // First handle drag event
      for event in mouse_event_channel.read(reader) {
        match event {
          MouseEvent::DragMove(rel_pos, _) => {
            let delta : Vector2 = (*rel_pos).into();
            let movement = vec2![-delta.x, delta.y] * viewport.virtual_to_screen_scale();
            viewport_event_channel.single_write(ViewportEvent::Move(movement));
          },
          _ => (),
        }
      }

      // TODO: Then handle
      // if input_state.rel_scroll.y != 0.0 {
      //   let virt_mouse_pos : Vector2 = input_state.mouse_abs_pos.to_virtual(&viewport).into();
      //   let mouse_to_center = viewport.virtual_center - virt_mouse_pos;

      //   let delta = input_state.rel_scroll.y * 0.01;
      //   let sign = delta.signum();

      //   let new_mouse_to_center = mouse_to_center + mouse_to_center.normalized() * delta;
      //   let diff = mouse_to_center - new_mouse_to_center;

      //   viewport_event_channel.single_write(ViewportEvent::Scale(diff.magnitude() * -sign));
      //   viewport_event_channel.single_write(ViewportEvent::Move(diff));
      // }
    }
  }
}