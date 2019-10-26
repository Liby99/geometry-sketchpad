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
use core_lib::math::*;
use core_ui::{resources::ExitState, setup_core_ui};

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
    builder.add_thread_local(SenderSystem { sender: render_update_tx });
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
      RenderUpdateEvent::None => {
        let event_name = cx.string("none");
        o.set(&mut cx, "event", event_name)?;
      },
      _ => (), // TODO
    }
    Ok(o.upcast())
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