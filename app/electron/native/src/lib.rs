#[macro_use] extern crate core_lib;
extern crate core_ui;
extern crate specs;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use neon::context::Context;
use neon::task::Task;
use neon::types::{JsFunction, JsUndefined, JsNumber};
use neon::{declare_types, register_module};

use specs::prelude::*;
use core_lib::math::*;
use core_ui::{resources::*, setup_core_ui};

pub mod events;
pub mod systems;
pub mod utilities;

use events::*;
use systems::*;
use utilities::*;

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
        emitter.receiver.send(UserEvent::Input(InputEvent::Button(ButtonState::Press, Button::Keyboard(u32_to_key(k)))))
      }).or_else(|err| cx.throw_error(&err.to_string()))?;
      Ok(JsUndefined::new().upcast())
    }

    method onKeyUp(mut cx) {
      let this = cx.this();
      let k = cx.argument::<JsNumber>(0)?.value() as u32;
      cx.borrow(&this, |emitter| {
        emitter.receiver.send(UserEvent::Input(InputEvent::Button(ButtonState::Release, Button::Keyboard(u32_to_key(k)))))
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

static CONSTANTS : [(&'static str, u32); 16] = [
  ("EVENT_TYPE_NONE", 0),
  ("EVENT_TYPE_INSERTED_POINT", 1),
  ("EVENT_TYPE_INSERTED_LINE", 2),
  ("EVENT_TYPE_INSERTED_CIRCLE", 3),
  ("EVENT_TYPE_INSERTED_RECTANGLE", 4),
  ("EVENT_TYPE_UPDATED_POINT", 5),
  ("EVENT_TYPE_UPDATED_LINE", 6),
  ("EVENT_TYPE_UPDATED_CIRCLE", 7),
  ("EVENT_TYPE_UPDATED_RECTANGLE", 8),
  ("EVENT_TYPE_UPDATED_POINT_STYLE", 9),
  ("EVENT_TYPE_UPDATED_LINE_STYLE", 10),
  ("EVENT_TYPE_UPDATED_CIRCLE_STYLE", 11),
  ("EVENT_TYPE_UPDATED_RECTANGLE_STYLE", 12),
  ("EVENT_TYPE_REMOVED_ENTITY", 13),
  ("EVENT_TYPE_SELECTED_ENTITY", 14),
  ("EVENT_TYPE_DESELECTED_ENTITY", 15),
];

register_module!(mut cx, {
  cx.export_class::<JsEventEmitter>("GeopadWorld")?;

  for (event_type, number) in &CONSTANTS {
    let js_number = cx.number(*number);
    cx.export_value::<JsNumber>(event_type, js_number)?;
  }

  Ok(())
});