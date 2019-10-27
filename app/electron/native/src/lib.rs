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
use neon::types::{JsFunction, JsUndefined, JsValue, JsNumber, JsString};
use neon::{declare_types, register_module};

use specs::prelude::*;
use core_lib::{math::*};
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

    // Creates an object of the shape `{ "event": string, ...data }`
    match event {
      RenderUpdateEvent::None => (),
      RenderUpdateEvent::SelectedEntity(ent) => {
        // TODO
      },
      RenderUpdateEvent::DeselectedEntity(ent) => {
        // TODO
      },
      RenderUpdateEvent::RemovedEntity(ent) => {
        // TODO
      },
      RenderUpdateEvent::UpdatedPoint(ent, sym_point, point_style) => {
        let name = cx.string("update-point");
        let entity = cx.number(ent.0);
      },
    }
    Ok(o.upcast())
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