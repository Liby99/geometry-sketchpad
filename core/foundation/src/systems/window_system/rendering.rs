use piston_window::{Event as PistonEvent, *};
use specs::prelude::*;
use geopad_core_lib::{
  math::*,
  utilities::*,
  components::{screen_shapes::*, styles::*, markers::*}
};

pub fn render<'a>(
  window: &mut PistonWindow,
  event: &PistonEvent,
  scrn_points: &ReadStorage<'a, ScreenPoint>,
  scrn_lines: &ReadStorage<'a, ScreenLine>,
  scrn_circles: &ReadStorage<'a, ScreenCircle>,
  scrn_rects: &ReadStorage<'a, ScreenRectangle>,
  point_styles: &ReadStorage<'a, PointStyle>,
  line_styles: &ReadStorage<'a, LineStyle>,
  circle_styles: &ReadStorage<'a, CircleStyle>,
  rect_styles: &ReadStorage<'a, RectangleStyle>,
  selecteds: &ReadStorage<'a, Selected>,
  hiddens: &ReadStorage<'a, Hidden>,
) {
  window.draw_2d(event, |context, graphics, _device| {

    // Clean the screen first
    clear(Color::white().into(), graphics);

    // NOTE: The later we draw, the higher the shape will be in the layers
    // i.e. The later we draw, the shape will be more on top of other shapes
    // Therefore we first draw circle, then line, then point, then rectangles
    // As circle should be at the bottom, line next, and point should be on the top
    // Note that currently we only have select rectangles so we draw rectangles on the most
    // top.

    // First draw the circles
    for (circle, style, _, _) in (scrn_circles, circle_styles, !selecteds, !hiddens).join() {
      render_circle(circle, style, false, context, graphics);
    }
    for (circle, style, _, _) in (scrn_circles, circle_styles, selecteds, !hiddens).join() {
      render_circle(circle, style, true, context, graphics);
    }

    // Then, draw the lines
    for (line, style, _, _) in (scrn_lines, line_styles, !selecteds, !hiddens).join() {
      render_line(line, style, false, context, graphics);
    }
    for (line, style, _, _) in (scrn_lines, line_styles, selecteds, !hiddens).join() {
      render_line(line, style, true, context, graphics);
    }

    // Lastly, draw the points
    for (point, style, _, _) in (scrn_points, point_styles, !selecteds, !hiddens).join() {
      render_point(point, style, false, context, graphics);
    }
    for (point, style, _, _) in (scrn_points, point_styles, selecteds, !hiddens).join() {
      render_point(point, style, true, context, graphics);
    }

    // Additionally, draw rectangles
    for (rect, style) in (scrn_rects, rect_styles).join() {
      render_rectangle(rect, style, context, graphics);
    }
  });
}

fn render_point(
  ScreenPosition(Vector2 { x, y }): &ScreenPoint,
  style: &PointStyle,
  selected: bool,
  context: Context,
  graphics: &mut G2d,
) {
  if selected {
    let radius = style.radius + 3.0;
    circle_arc(
      Color::magenta().into(),
      0.5,
      0.0,
      std::f64::consts::PI * 1.9999,
      [x - radius, y - radius, radius * 2., radius * 2.],
      context.transform,
      graphics
    );
  }
  ellipse(
    rgba!(0.0, 0.0, 0.0, style.color.a).into(),
    [x - style.radius, y - style.radius, style.radius * 2., style.radius * 2.],
    context.transform,
    graphics,
  );
  let center_radius = style.radius - 1.5;
  ellipse(
    style.color.into(),
    [x - center_radius, y - center_radius, center_radius * 2., center_radius * 2.],
    context.transform,
    graphics,
  );
}

fn render_line(
  ScreenLine { from, to, .. }: &ScreenLine,
  style: &LineStyle,
  selected: bool,
  context: Context,
  graphics: &mut G2d,
) {
  let from : Vector2 = Into::<Vector2>::into(*from);
  let to : Vector2 = Into::<Vector2>::into(*to);
  line_from_to(style.color.into(), style.width, from, to, context.transform, graphics);
  if selected {
    let Vector2 { x: dx, y: dy } = (to - from).normalized();
    let perp_dir = vec2![-dy, dx] * (style.width / 2.0 + 3.0);
    line_from_to(Color::magenta().into(), 0.5, from - perp_dir, to - perp_dir, context.transform, graphics);
    line_from_to(Color::magenta().into(), 0.5, from + perp_dir, to + perp_dir, context.transform, graphics);
  }
}

fn render_circle(
  ScreenCircle { center, radius }: &ScreenCircle,
  style: &CircleStyle,
  selected: bool,
  context: Context,
  graphics: &mut G2d,
) {
  let center : Vector2 = Into::<Vector2>::into(*center);
  let radius : f64 = Into::<f64>::into(*radius);
  let rect = [center.x - radius, center.y - radius, radius * 2.0, radius * 2.0];
  ellipse(
    style.fill.into(),
    rect,
    context.transform,
    graphics
  );
  circle_arc(
    style.border.color.into(),
    style.border.width,
    0.0,
    std::f64::consts::PI * 1.999999999,
    rect,
    context.transform,
    graphics,
  );
  if selected {
    let inner_radius = radius - style.border.width / 2.0 - 3.0;
    let outer_radius = radius + style.border.width / 2.0 + 3.0;
    circle_arc(
      Color::magenta().into(),
      0.5,
      0.0,
      std::f64::consts::PI * 1.999999999,
      [
        center.x - inner_radius,
        center.y - inner_radius,
        inner_radius * 2.0,
        inner_radius * 2.0,
      ],
      context.transform,
      graphics,
    );
    circle_arc(
      Color::magenta().into(),
      0.5,
      0.0,
      std::f64::consts::PI * 1.999999999,
      [
        center.x - outer_radius,
        center.y - outer_radius,
        outer_radius * 2.0,
        outer_radius * 2.0,
      ],
      context.transform,
      graphics,
    )
  }
}

fn render_rectangle(rect: &AABB, style: &RectangleStyle, context: Context, graphics: &mut G2d) {
  line_from_to(style.border.color.into(), style.border.width, [rect.x, rect.y], [rect.x, rect.y + rect.height], context.transform, graphics);
  line_from_to(style.border.color.into(), style.border.width, [rect.x, rect.y], [rect.x + rect.width, rect.y], context.transform, graphics);
  line_from_to(style.border.color.into(), style.border.width, [rect.x + rect.width, rect.y], [rect.x + rect.width, rect.y + rect.height], context.transform, graphics);
  line_from_to(style.border.color.into(), style.border.width, [rect.x, rect.y + rect.height], [rect.x + rect.width, rect.y + rect.height], context.transform, graphics);
  rectangle(style.fill.into(), [rect.x, rect.y, rect.width, rect.height], context.transform, graphics);
}