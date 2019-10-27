#[macro_use] extern crate core_lib;
extern crate core_ui;
extern crate specs;

use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use neon::context::{Context, TaskContext};
use neon::object::Object;
use neon::result::JsResult;
use neon::task::Task;
use neon::types::{JsFunction, JsUndefined, JsValue, JsNumber};
use neon::{declare_types, register_module};

use specs::prelude::*;
use core_lib::{math::*, utilities::*, components::styles::*};
use core_ui::{resources::*, setup_core_ui};

pub mod output;
pub mod input;
pub mod sender_system;
pub mod receiver_system;

use output::*;
use input::*;
use sender_system::*;
use receiver_system::*;

fn event_thread(user_event_rx: mpsc::Receiver<UserEvent>) -> mpsc::Receiver<RenderUpdateEvent> {

  // Create sending and receiving channels for the event data
  let (render_update_tx, render_update_rx) = mpsc::channel();

  // Spawn a thead to continue running after this method has returned.
  thread::spawn(move || {

    // Generate a new world
    let mut world = World::new();
    let mut builder = DispatcherBuilder::new();

    // Setup the core ui
    setup_core_ui(&mut builder);

    // Add the sender and receiver system
    builder.add_thread_local(SenderSystem::new(render_update_tx));
    builder.add_thread_local(ReceiverSystem { receiver: user_event_rx });

    // Build the dispatcher
    let mut dispatcher = builder.build();
    dispatcher.setup(&mut world);

    // Enter game loop
    while !world.fetch::<ExitState>().is_exiting() {
      dispatcher.dispatch(&mut world);
    }
  });

  render_update_rx
}

pub struct EventEmitterTask(Arc<Mutex<mpsc::Receiver<RenderUpdateEvent>>>);

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
    let event_type = cx.number(type_to_u32(&event));
    o.set(&mut cx, "type", event_type)?;

    // Creates an object of the shape `{ "event": string, ...data }`
    match event {
      RenderUpdateEvent::None => (),
      RenderUpdateEvent::SelectedEntity(ent) => {
        let event_entity = cx.string(format!("{}_{}", ent.id(), ent.gen().id()));
        o.set(&mut cx, "entity", event_entity)?;
      },
      RenderUpdateEvent::DeselectedEntity(ent) => {
        let event_entity = cx.string(format!("{}_{}", ent.id(), ent.gen().id()));
        o.set(&mut cx, "entity", event_entity)?;
      },
      RenderUpdateEvent::RemovedEntity(ent) => {
        let event_entity = cx.string(format!("{}_{}", ent.id(), ent.gen().id()));
        o.set(&mut cx, "entity", event_entity)?;
      },
      RenderUpdateEvent::UpdatedPoint(ent, ScreenPosition(Vector2 { x, y }), PointStyle { color, radius, border_color, border_width }) => {
        let event_entity = cx.string(format!("{}_{}", ent.id(), ent.gen().id()));
        o.set(&mut cx, "entity", event_entity)?;

        let event_scrn_point = cx.empty_object();
        let event_scrn_point_x = cx.number(x);
        let event_scrn_point_y = cx.number(y);
        event_scrn_point.set(&mut cx, "x", event_scrn_point_x)?;
        event_scrn_point.set(&mut cx, "y", event_scrn_point_y)?;
        o.set(&mut cx, "position", event_scrn_point)?;

        let event_style = cx.empty_object();
        let event_style_color = cx.number(color_to_hex(color));
        let event_style_alpha = cx.number(color.a);
        let event_style_border_color = cx.number(color_to_hex(border_color));
        let event_style_border_alpha = cx.number(border_color.a);
        let event_style_radius = cx.number(radius);
        let event_style_border_width = cx.number(border_width);
        event_style.set(&mut cx, "color", event_style_color)?;
        event_style.set(&mut cx, "alpha", event_style_alpha)?;
        event_style.set(&mut cx, "borderColor", event_style_border_color)?;
        event_style.set(&mut cx, "borderAlpha", event_style_border_alpha)?;
        event_style.set(&mut cx, "radius", event_style_radius)?;
        event_style.set(&mut cx, "borderWidth", event_style_border_width)?;
        o.set(&mut cx, "style", event_style)?;
      },
    }
    Ok(o.upcast())
  }
}

pub fn color_to_hex(color: Color) -> u32 {
  (((color.r * 255.0) as u32) << 16) | (((color.g * 255.0) as u32) << 8) | (color.b * 255.0) as u32
}

pub fn type_to_u32(event: &RenderUpdateEvent) -> u32 {
  match event {
    RenderUpdateEvent::None => 0,
    RenderUpdateEvent::UpdatedPoint(_, _, _) => 1,
    RenderUpdateEvent::RemovedEntity(_) => 4,
    RenderUpdateEvent::SelectedEntity(_) => 5,
    RenderUpdateEvent::DeselectedEntity(_) => 6,
  }
}

pub fn to_key(k: u32) -> Key {
  match k {
    8 => Key::Delete,
    16 => Key::LShift,
    17 => Key::LCtrl,
    18 => Key::LAlt,
    32 => Key::Space,
    48 => Key::D0,
    49 => Key::D1,
    50 => Key::D2,
    51 => Key::D3,
    52 => Key::D4,
    53 => Key::D5,
    54 => Key::D6,
    55 => Key::D7,
    56 => Key::D8,
    57 => Key::D9,
    65 => Key::A,
    66 => Key::B,
    67 => Key::C,
    68 => Key::D,
    69 => Key::E,
    70 => Key::F,
    71 => Key::G,
    72 => Key::H,
    73 => Key::I,
    74 => Key::J,
    75 => Key::K,
    76 => Key::L,
    77 => Key::M,
    78 => Key::N,
    79 => Key::O,
    80 => Key::P,
    81 => Key::Q,
    82 => Key::R,
    83 => Key::S,
    84 => Key::T,
    85 => Key::U,
    86 => Key::V,
    87 => Key::W,
    88 => Key::X,
    89 => Key::Y,
    90 => Key::Z,
    91 => Key::LCommand,
    93 => Key::RCommand,
    187 => Key::Equals,
    189 => Key::Minus,
    220 => Key::Backslash,
    _ => Key::Unknown,
  }
}

pub struct EventEmitter {
  emitter: Arc<Mutex<mpsc::Receiver<RenderUpdateEvent>>>,
  receiver: mpsc::Sender<UserEvent>,
}

declare_types! {
  pub class JsEventEmitter for EventEmitter {
    init(_) {
      let (receiver, user_event_rx) = mpsc::channel();
      let rx = event_thread(user_event_rx);
      Ok(EventEmitter { emitter: Arc::new(Mutex::new(rx)), receiver })
    }

    method poll(mut cx) {
      let cb = cx.argument::<JsFunction>(0)?;
      let this = cx.this();
      let events = cx.borrow(&this, |emitter| Arc::clone(&emitter.emitter));
      let emitter = EventEmitterTask(events);
      emitter.schedule(cb);
      Ok(JsUndefined::new().upcast())
    }

    method step(mut cx) {
      let this = cx.this();
      cx.borrow(&this, |emitter| emitter.receiver.send(UserEvent::Loop)).or_else(|err| cx.throw_error(&err.to_string()))?;
      Ok(JsUndefined::new().upcast())
    }

    method onMouseMove(mut cx) {
      let this = cx.this();
      let x = cx.argument::<JsNumber>(0)?.value() as f64;
      let y = cx.argument::<JsNumber>(1)?.value() as f64;
      let rel_x = cx.argument::<JsNumber>(2)?.value() as f64;
      let rel_y = cx.argument::<JsNumber>(3)?.value() as f64;
      cx.borrow(&this, |emitter| {
        emitter.receiver.send(UserEvent::Input(InputEvent::Motion(MotionEvent::MouseCursor(vec2![x, y]))))?;
        emitter.receiver.send(UserEvent::Input(InputEvent::Motion(MotionEvent::MouseRelative(vec2![rel_x, rel_y]))))
      }).or_else(|err| cx.throw_error(&err.to_string()))?;
      Ok(JsUndefined::new().upcast())
    }

    method onMouseDown(mut cx) {
      let this = cx.this();
      cx.borrow(&this, |emitter| {
        emitter.receiver.send(UserEvent::Input(InputEvent::Button(ButtonState::Press, Button::Mouse(MouseButton::Left))))
      }).or_else(|err| cx.throw_error(&err.to_string()))?;
      Ok(JsUndefined::new().upcast())
    }

    method onMouseUp(mut cx) {
      let this = cx.this();
      cx.borrow(&this, |emitter| {
        emitter.receiver.send(UserEvent::Input(InputEvent::Button(ButtonState::Release, Button::Mouse(MouseButton::Left))))
      }).or_else(|err| cx.throw_error(&err.to_string()))?;
      Ok(JsUndefined::new().upcast())
    }

    method onKeyDown(mut cx) {
      let this = cx.this();
      let k = cx.argument::<JsNumber>(0)?.value() as u32;
      cx.borrow(&this, |emitter| {
        emitter.receiver.send(UserEvent::Input(InputEvent::Button(ButtonState::Press, Button::Keyboard(to_key(k)))))
      }).or_else(|err| cx.throw_error(&err.to_string()))?;
      Ok(JsUndefined::new().upcast())
    }

    method onKeyUp(mut cx) {
      let this = cx.this();
      let k = cx.argument::<JsNumber>(0)?.value() as u32;
      cx.borrow(&this, |emitter| {
        emitter.receiver.send(UserEvent::Input(InputEvent::Button(ButtonState::Release, Button::Keyboard(to_key(k)))))
      }).or_else(|err| cx.throw_error(&err.to_string()))?;
      Ok(JsUndefined::new().upcast())
    }

    method shutdown(mut cx) {
      let this = cx.this();
      cx.borrow(&this, |emitter| emitter.receiver.send(UserEvent::Shutdown)).or_else(|err| cx.throw_error(&err.to_string()))?;
      Ok(JsUndefined::new().upcast())
    }
  }
}

register_module!(mut cx, {
  cx.export_class::<JsEventEmitter>("GeopadWorld")?;
  Ok(())
});