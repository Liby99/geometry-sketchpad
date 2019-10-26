use specs::prelude::*;
use core_ui::{events::*, resources::*};
use super::input::*;

static CLICK_TIME_THRESHOLD : u128 = 100; // 0.1 second

pub struct ReceiverSystem {
  pub receiver: std::sync::mpsc::Receiver<UserEvent>,
}

impl<'a> System<'a> for ReceiverSystem {
  type SystemData = (
    Write<'a, InputState>,
    Write<'a, MouseEventChannel>,
    Write<'a, ExitEventChannel>,
  );

  fn run(&mut self, (
    mut input_state,
    mut mouse_event_channel,
    mut exit_event_channel,
  ): Self::SystemData) {
    input_state.reset_relative_data();
    loop {
      match self.receiver.try_recv() {
        Ok(user_event) => match user_event {
          UserEvent::Loop => {
            break;
          },
          UserEvent::Input(input) => match input {
            InputEvent::Motion(motion) => match motion {
              MotionEvent::MouseCursor(abs_pos) => {
                println!("mouse move {:?}", abs_pos);
                input_state.mouse_abs_pos = abs_pos.into();
              },
              MotionEvent::MouseRelative(rel_mov) => {
                input_state.mouse_rel_movement = input_state.mouse_rel_movement + rel_mov.into();
              },
              MotionEvent::MouseScroll(rel_scroll) => {
                input_state.rel_scroll = input_state.rel_scroll + rel_scroll;
              },
            },
            InputEvent::Button(button_state, button) => {
              let is_pressed = button_state == ButtonState::Press;
              match button {
                Button::Keyboard(key) => {
                  input_state.keyboard.set(key, is_pressed);
                },
                Button::Mouse(mouse_button) => match mouse_button {
                  MouseButton::Left => {
                    println!("mouse button {}", is_pressed);
                    input_state.mouse_left_button.set(is_pressed);
                  },
                  MouseButton::Right => {
                    input_state.mouse_right_button.set(is_pressed);
                  },
                },
              }
            },
          },
          UserEvent::Shutdown => {
            exit_event_channel.single_write(ExitEvent);
          },
        },
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
          exit_event_channel.single_write(ExitEvent);
        },
        Err(std::sync::mpsc::TryRecvError::Empty) => (),
      }
    }
  }
}