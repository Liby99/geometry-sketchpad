use specs::prelude::*;
use crate::{
  math::Vector2,
  resources::{Viewport, InputState, ToolState},
  components::{Point, /*Line, */Selected},
};

static SELECT_DIST_THRES : f64 = 5.0; // Pixel

pub struct SelectPointSystem;

impl<'a> System<'a> for SelectPointSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolState>,
    Read<'a, InputState>,
    Read<'a, Viewport>,
    ReadStorage<'a, Point>,
    WriteStorage<'a, Selected>,
  );

  fn run(&mut self, (
    entities,
    tool,
    input,
    vp,
    points,
    mut selected,
  ): Self::SystemData) {
    match *tool {
      ToolState::Select => {
        if input.mouse_left_button.just_activated() {
          let mouse_pos = Vector2::from(input.mouse_abs_pos);

          // TODO: CHange this logic to getting the closest point & Make this point size dependent
          for (ent, p) in (&entities, &points).join() {
            if (Vector2::from(vp.to_actual(*p)) - mouse_pos).magnitude() <= SELECT_DIST_THRES {
              match selected.get(ent) {
                Some(_) => { selected.remove(ent); },
                None => if let Err(err) = selected.insert(ent, Selected) {
                  panic!("Error selecting {:?}: {}", ent, err);
                },
              }
              return;
            }
          }

          // If nothing selected, clear the selection
          selected.clear();
        }
      },
      _ => (),
    }
  }
}

// pub struct SelectLineSystem;

// impl<'a> System<'a> for SelectLineSystem {
//   type SystemData = (
//     ReadStorage<'a, Line>,
//     WriteStorage<'a, Selected>,
//   );

//   fn run(&mut self, (lines, selected): Self::SystemData) {

//   }
// }