use specs::*;
use nfd::Response;
use crate::{
  utilities::Key,
  resources::InputState,
};

pub struct OpenFileViaKeyboard;

impl<'a> System<'a> for OpenFileViaKeyboard {
  type SystemData = Read<'a, InputState>;

  fn run(&mut self, input_state: Self::SystemData) {
    if input_state.keyboard.just_activated(Key::O) && input_state.keyboard.is_command_activated() {
      match nfd::open_file_dialog(None, None) {
        Ok(result) => match result {
          Response::Okay(file_path) => println!("File path = {:?}", file_path),
          Response::OkayMultiple(files) => println!("Files {:?}", files),
          Response::Cancel => println!("User canceled"),
        },
        Err(err) => println!("Error: {:?}", err),
      }
    }
  }
}