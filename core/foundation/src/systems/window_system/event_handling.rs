use std::time::SystemTime;
use specs::prelude::*;
use piston_window::{Input, ButtonArgs, ButtonState, Button, MouseButton, Motion, ResizeArgs};
use geopad_core_lib::{math::*, events::*};
use crate::{events::*, resources::*};

static CLICK_TIME_THRESHOLD : u128 = 100; // 0.1 second

pub fn handle_input<'a>(
  input: Input,
  input_state: &mut Write<'a, InputState>,
  mouse_event_channel: &mut Write<'a, MouseEventChannel>,
  viewport_event_channel: &mut Write<'a, ViewportEventChannel>,
) {
  input_state.reset_relative_data();
  match input {
    Input::Button(ButtonArgs { state, button, scancode }) => {
      let is_pressed = state == ButtonState::Press;
      match button {
        Button::Mouse(MouseButton::Left) => {
          input_state.mouse_left_button.set(is_pressed);
          if is_pressed {
            input_state.mouse_left_button_last_pressed = Some(SystemTime::now());
            mouse_event_channel.single_write(MouseEvent::MouseDown(input_state.mouse_abs_pos));
          } else {
            mouse_event_channel.single_write(MouseEvent::MouseUp(input_state.mouse_abs_pos));
            if input_state.is_mouse_left_button_dragging {
              input_state.is_mouse_left_button_dragging = false;
              mouse_event_channel.single_write(MouseEvent::DragEnd(input_state.mouse_abs_pos));
            } else {
              if let Some(last_pressed) = input_state.mouse_left_button_last_pressed {
                if last_pressed.elapsed().unwrap().as_millis() < CLICK_TIME_THRESHOLD {
                  mouse_event_channel.single_write(MouseEvent::Click(input_state.mouse_abs_pos));
                }
              }
            }
          }
        },
        Button::Mouse(MouseButton::Right) => {
          input_state.mouse_right_button.set(is_pressed);
        },
        Button::Keyboard(piston_key) => {
          let key = Key::from((piston_key, scancode));
          input_state.keyboard.set(key, is_pressed)
        },
        _ => (),
      }
    },
    Input::Move(motion) => {
      match motion {
        Motion::MouseScroll(rel_scroll) => {
          input_state.rel_scroll = input_state.rel_scroll + rel_scroll.into()
        },
        Motion::MouseCursor(abs_pos) => {
          input_state.mouse_abs_pos = From::<Vector2>::from(abs_pos.into())
        },
        Motion::MouseRelative(rel_mov) => {
          let scrn_rel_mov = From::<Vector2>::from(rel_mov.into());
          input_state.mouse_rel_movement = input_state.mouse_rel_movement + scrn_rel_mov;
          if input_state.is_mouse_left_button_dragging {
            mouse_event_channel.single_write(MouseEvent::DragMove(scrn_rel_mov, input_state.mouse_abs_pos));
          } else {
            if input_state.mouse_left_button.is_activated() {
              input_state.is_mouse_left_button_dragging = true;
              mouse_event_channel.single_write(MouseEvent::DragBegin(input_state.mouse_abs_pos));
            }
          }
        },
        _ => (),
      }
    },
    Input::Resize(ResizeArgs { window_size, .. }) => {
      viewport_event_channel.single_write(ViewportEvent::Resize(Vector2::from(window_size)));
    },
    _ => (),
  }
}

pub fn handle_dt_update<'a>(dt: f64, delta_time: &mut Write<'a, DeltaTime>) {
  delta_time.set(dt);
}