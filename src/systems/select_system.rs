use specs::prelude::*;
use crate::{
  resources::{ToolState},
  components::{
    point::Point,
    line::Line,
    select::Selected
  }
};

pub struct SelectPointSystem;

impl<'a> System<'a> for SelectPointSystem {
  type SystemData = (
    Read<'a, ToolState>,
    // Read<'a, InputEvents>,
    ReadStorage<'a, Point>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (tool, /*inputs,*/ points, selected): Self::SystemData) {
    match *tool {
      ToolState::Select => {
        // match inputs.peek() {
        //   _ => (),
        // }
      },
      _ => (),
    }

  }
}

pub struct SelectLineSystem;

impl<'a> System<'a> for SelectLineSystem {
  type SystemData = (
    ReadStorage<'a, Line>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (lines, selected): Self::SystemData) {

  }
}