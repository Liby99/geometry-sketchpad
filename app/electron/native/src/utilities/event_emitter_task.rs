use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use neon::context::{Context, TaskContext};
use neon::object::Object;
use neon::result::JsResult;
use neon::task::Task;
use neon::types::{JsUndefined, JsValue};

use core_lib::{math::*, utilities::*, components::styles::*};
use crate::events::*;
use super::*;

pub struct EventEmitterTask(pub Arc<Mutex<mpsc::Receiver<RenderUpdateEvent>>>);

impl Task for EventEmitterTask {
  type Output = Option<RenderUpdateEvent>;
  type Error = String;
  type JsEvent = JsValue;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let rx = self.0.lock().map_err(|_| "Could not obtain lock on receiver".to_string())?;

    match rx.recv_timeout(Duration::from_millis(100)) {
      Ok(event) => Ok(Some(event)),
      Err(RecvTimeoutError::Timeout) => Ok(None),
      Err(RecvTimeoutError::Disconnected) => Err("Failed to receive event".to_string()),
    }
  }

  fn complete(
    self,
    mut cx: TaskContext,
    event: Result<Self::Output, Self::Error>,
  ) -> JsResult<Self::JsEvent> {

    // Receive the event or return early with the error
    let event = event.or_else(|err| cx.throw_error(&err.to_string()))?;

    // Timeout occured, return early with `undefined
    let event = match event {
      Some(event) => event,
      None => return Ok(JsUndefined::new().upcast()),
    };

    // Create an empty object `{}`
    let o = cx.empty_object();
    let event_type = cx.number(render_update_event_to_u32(&event));
    o.set(&mut cx, "type", event_type)?;

    // Macro rules to make my life easier
    macro_rules! entity {
      ($ent: expr) => {{
        let ent = $ent;
        cx.string(format!("{}_{}", ent.id(), ent.gen().id()))
      }};
    }

    macro_rules! position {
      ($pos: expr) => {{
        let ScreenPosition(Vector2 { x, y }) = $pos;
        let event_scrn_point = cx.empty_object();
        let event_scrn_point_x = cx.number(x);
        let event_scrn_point_y = cx.number(y);
        event_scrn_point.set(&mut cx, "x", event_scrn_point_x)?;
        event_scrn_point.set(&mut cx, "y", event_scrn_point_y)?;
        event_scrn_point
      }};
    }

    macro_rules! point_style {
      ($point_style: expr) => {{
        let PointStyle { color, radius, border_color, border_width } = $point_style;
        let style = cx.empty_object();
        let event_style_color = cx.number(color_to_hex(color));
        let event_style_alpha = cx.number(color.a);
        let event_style_border_color = cx.number(color_to_hex(border_color));
        let event_style_border_alpha = cx.number(border_color.a);
        let event_style_radius = cx.number(radius);
        let event_style_border_width = cx.number(border_width);
        style.set(&mut cx, "color", event_style_color)?;
        style.set(&mut cx, "alpha", event_style_alpha)?;
        style.set(&mut cx, "borderColor", event_style_border_color)?;
        style.set(&mut cx, "borderAlpha", event_style_border_alpha)?;
        style.set(&mut cx, "radius", event_style_radius)?;
        style.set(&mut cx, "borderWidth", event_style_border_width)?;
        style
      }};
    }

    macro_rules! line {
      ($line: expr) => {{
        let ScreenLine { from, to, .. } = $line;
        let line = cx.empty_object();
        let from = position!(from);
        let to = position!(to);
        line.set(&mut cx, "from", from)?;
        line.set(&mut cx, "to", to)?;
        line
      }};
    }

    macro_rules! line_style {
      ($line_style: expr) => {{
        let LineStyle { color, width } = $line_style;
        let rgb = cx.number(color_to_hex(color));
        let alpha = cx.number(color.a);
        let width = cx.number(width);
        let style = cx.empty_object();
        style.set(&mut cx, "color", rgb)?;
        style.set(&mut cx, "alpha", alpha)?;
        style.set(&mut cx, "width", width)?;
        style
      }};
    }

    macro_rules! circle {
      ($circle: expr) => {{
        let ScreenCircle { center, radius } = $circle;
        let circle = cx.empty_object();
        let center = position!(center);
        let radius = cx.number(radius);
        circle.set(&mut cx, "center", center)?;
        circle.set(&mut cx, "radius", radius)?;
        circle
      }};
    }

    macro_rules! circle_style {
      ($circle_style: expr) => {{
        let CircleStyle { fill, border } = $circle_style;
        let circle_style = cx.empty_object();
        let fill_rgb = cx.number(color_to_hex(fill));
        let fill_alpha = cx.number(fill.a);
        let border = line_style!(border);
        circle_style.set(&mut cx, "fill", fill_rgb)?;
        circle_style.set(&mut cx, "fillAlpha", fill_alpha)?;
        circle_style.set(&mut cx, "border", border)?;
        circle_style
      }};
    }

    macro_rules! rect {
      ($rect: expr) => {{
        let AABB { x, y, width, height } = $rect;
        let rect = cx.empty_object();
        let x = cx.number(x);
        let y = cx.number(y);
        let width = cx.number(width);
        let height = cx.number(height);
        rect.set(&mut cx, "x", x)?;
        rect.set(&mut cx, "y", y)?;
        rect.set(&mut cx, "width", width)?;
        rect.set(&mut cx, "height", height)?;
        rect
      }};
    }

    macro_rules! rect_style {
      ($rect_style: expr) => {{
        let RectangleStyle { fill, border } = $rect_style;
        let rect_style = cx.empty_object();
        let fill_rgb = cx.number(color_to_hex(fill));
        let fill_alpha = cx.number(fill.a);
        let border = line_style!(border);
        rect_style.set(&mut cx, "fill", fill_rgb)?;
        rect_style.set(&mut cx, "fillAlpha", fill_alpha)?;
        rect_style.set(&mut cx, "border", border)?;
        rect_style
      }};
    }

    match event {
      RenderUpdateEvent::None => (),
      RenderUpdateEvent::InsertedPoint(ent, scrn_point, point_style) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let position = position!(scrn_point);
        o.set(&mut cx, "point", position)?;
        let style = point_style!(point_style);
        o.set(&mut cx, "style", style)?;
      },
      RenderUpdateEvent::InsertedLine(ent, scrn_line, line_style) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let line = line!(scrn_line);
        o.set(&mut cx, "line", line)?;
        let style = line_style!(line_style);
        o.set(&mut cx, "style", style)?;
      },
      RenderUpdateEvent::InsertedCircle(ent, scrn_circle, circle_style) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let circle = circle!(scrn_circle);
        o.set(&mut cx, "circle", circle)?;
        let style = circle_style!(circle_style);
        o.set(&mut cx, "style", style)?;
      },
      RenderUpdateEvent::InsertedRectangle(ent, scrn_rect, rect_style) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let rect = rect!(scrn_rect);
        o.set(&mut cx, "rect", rect)?;
        let style = rect_style!(rect_style);
        o.set(&mut cx, "style", style)?;
      },
      RenderUpdateEvent::UpdatedPoint(ent, scrn_point) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let position = position!(scrn_point);
        o.set(&mut cx, "point", position)?;
      },
      RenderUpdateEvent::UpdatedLine(ent, scrn_line) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let line = line!(scrn_line);
        o.set(&mut cx, "line", line)?;
      },
      RenderUpdateEvent::UpdatedCircle(ent, scrn_circle) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let circle = circle!(scrn_circle);
        o.set(&mut cx, "circle", circle)?;
      },
      RenderUpdateEvent::UpdatedRectangle(ent, scrn_rect) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let rect = rect!(scrn_rect);
        o.set(&mut cx, "rect", rect)?;
      },
      RenderUpdateEvent::UpdatedPointStyle(ent, point_style) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let style = point_style!(point_style);
        o.set(&mut cx, "style", style)?;
      },
      RenderUpdateEvent::UpdatedLineStyle(ent, line_style) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let style = line_style!(line_style);
        o.set(&mut cx, "style", style)?;
      },
      RenderUpdateEvent::UpdatedCircleStyle(ent, circle_style) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let style = circle_style!(circle_style);
        o.set(&mut cx, "style", style)?;
      },
      RenderUpdateEvent::UpdatedRectangleStyle(ent, rect_style) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
        let style = rect_style!(rect_style);
        o.set(&mut cx, "style", style)?;
      },
      RenderUpdateEvent::SelectedEntity(ent) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
      },
      RenderUpdateEvent::DeselectedEntity(ent) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
      },
      RenderUpdateEvent::RemovedEntity(ent) => {
        let entity = entity!(ent);
        o.set(&mut cx, "entity", entity)?;
      },
    }
    Ok(o.upcast())
  }
}