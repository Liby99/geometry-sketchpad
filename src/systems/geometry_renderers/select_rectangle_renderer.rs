use specs::prelude::*;
use crate::{
  utilities::Color,
  resources::geometry::SelectRectangle,
  components::{Rectangle, RectangleStyle, LineStyle},
};

static SELECT_RECT_STYLE : RectangleStyle = RectangleStyle {
  border: LineStyle {
    color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.3 },
    width: 1.,
  },
  fill: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.05 },
};

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
    WriteStorage<'a, Rectangle>,
    WriteStorage<'a, RectangleStyle>,
  );

  fn run(&mut self, (entities, select_rect, mut rects, mut rect_styles): Self::SystemData) {

    // Make sure we have the rectangle entity
    let rect_ent = if let Some(ent) = self.drag_rectangle_entity { ent } else {
      let ent = entities.create();
      self.drag_rectangle_entity = Some(ent);
      if let Err(err) = rect_styles.insert(ent, SELECT_RECT_STYLE) { panic!(err) }
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