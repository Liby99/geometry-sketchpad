use std::time::SystemTime;
use specs::prelude::*;
use crate::{
  util::Vector2,
  resources::InputState,
  systems::events::{MouseEventChannel, MouseEvent, MouseEventReader},
};

static CLICK_TIME_THRESHOLD : f64 = 0.3; // In seconds

pub struct MouseEventEmitter {
  state: State,
}

enum State {
  UnTouched,
  Pressed(Vector2, SystemTime),
  Dragging(Vector2), // Starting position
  Released(SystemTime),
}

impl Default for MouseEventEmitter {
  fn default() -> Self {
    Self { state: State::UnTouched }
  }
}

impl<'a> System<'a> for MouseEventEmitter {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, MouseEventChannel>,
  );

  fn run(&mut self, (input_state, mut mouse_event_channel): Self::SystemData) {
    if input_state.mouse_left_button.just_activated() {

      // Pressed mouse left button just now
      self.state = State::Pressed(input_state.mouse_abs_pos, SystemTime::now());
      mouse_event_channel.single_write(MouseEvent::MouseDown(input_state.mouse_abs_pos));
    } else if input_state.mouse_left_button.just_deactivated() {

      match self.state {

        // Check if quickly clicked (Pressed and then released within CLICK_TIME_THRESHOLD)
        State::Pressed(_, press_time) => {
          if let Ok(dur) = press_time.elapsed() {
            if dur.as_secs_f64() <= CLICK_TIME_THRESHOLD {
              mouse_event_channel.single_write(MouseEvent::Click(input_state.mouse_abs_pos));
            }
          }
        },

        // Check if dragging. Then emit drag end event
        State::Dragging(_) => {
          mouse_event_channel.single_write(MouseEvent::DragEnd(input_state.mouse_abs_pos));
        },
        _ => (),
      }
      self.state = State::Released(SystemTime::now());
    } else {

      // This is the keep previous state event
      match self.state {
        State::Pressed(pos, _) => {

          // Check if already pressed and has relative movement. If so then is dragging.
          // Emit both drag begin event and move event
          if input_state.mouse_rel_movement.is_not_zero() {
            mouse_event_channel.single_write(MouseEvent::DragBegin(pos));
            mouse_event_channel.single_write(MouseEvent::DragMove(input_state.mouse_rel_movement));
            self.state = State::Dragging(pos);
          }
        },
        State::Dragging(_) => {

          // If is already dragging, emit drag move event
          if input_state.mouse_rel_movement.is_not_zero() {
            mouse_event_channel.single_write(MouseEvent::DragMove(input_state.mouse_rel_movement));
          }
        },
        _ => (),
      }
    }
  }
}

#[allow(dead_code)]
pub struct MouseEventDebugSystem {
  reader_id: Option<MouseEventReader>,
}

impl Default for MouseEventDebugSystem {
  fn default() -> Self {
    Self { reader_id: None }
  }
}

impl<'a> System<'a> for MouseEventDebugSystem {
  type SystemData = Read<'a, MouseEventChannel>;

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.reader_id = Some(world.fetch_mut::<MouseEventChannel>().register_reader());
  }

  fn run(&mut self, mouse_event_channel: Self::SystemData) {
    if let Some(reader_id) = &mut self.reader_id {
      for event in mouse_event_channel.read(reader_id) {
        println!("{:?}", event);
      }
    }
  }
}