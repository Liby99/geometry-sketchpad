use specs::prelude::*;
use geopad_core_lib::components::{screen_shapes::ScreenRectangle, styles::RectangleStyle};
use crate::resources::{DefaultSelectRectangleStyle, SelectRectangle};

pub struct SelectRectangleRenderer {
  drag_rectangle_entity: Option<Entity>,
}

impl Default for SelectRectangleRenderer {
  fn default() -> Self {
    Self { drag_rectangle_entity: None }
  }
}

impl<'a> System<'a> for SelectRectangleRenderer {
  type SystemData = (
    Entities<'a>,
    Read<'a, SelectRectangle>,
    Read<'a, DefaultSelectRectangleStyle>,
    WriteStorage<'a, ScreenRectangle>,
    WriteStorage<'a, RectangleStyle>,
  );

  fn run(&mut self, (entities, select_rect, select_rect_style, mut rects, mut rect_styles): Self::SystemData) {

    // Make sure we have the rectangle entity
    let rect_ent = if let Some(ent) = self.drag_rectangle_entity { ent } else {
      let ent = entities.create();
      self.drag_rectangle_entity = Some(ent);
      if let Err(err) = rect_styles.insert(ent, select_rect_style.get()) { panic!(err) }
      ent
    };

    // Check if there is select rectangle
    if let Some(rect) = select_rect.get() {

      // Add the rectangle if there's select_rect
      if let Err(err) = rects.insert(rect_ent, rect) { panic!(err) }
    } else {

      // Remove rectangle if there's not
      rects.remove(rect_ent);
    }
  }
}