use specs::prelude::*;
use nwg::{Event, Ui, dispatch_events};
use std::sync::Mutex;
use std::thread::spawn;
use crate::resources::{InputState, Tool, events::{ToolChangeEventChannel, ToolChangeEvent}};

#[derive(Debug, Clone, Hash)]
pub enum AppId {
  // parent window
  ToolWindow,

  // buttons
  SelectToolBtn,
  PointToolBtn,
  LineToolBtn,
  CircleToolBtn,
  ViewportDragToolBtn,

  // events
  SelectToolEvent,
  PointToolEvent,
  LineToolEvent,
  CircleToolEvent,
  ViewportDragToolEvent,

  // resources
  TextFont,
}

lazy_static! {
  static ref GUI_DATA : Mutex<Option<Tool>> = Mutex::new(None);
}

use AppId::*; // Shortcut

nwg_template!(
    head: setup_ui<AppId>,
    controls: [
        (ToolWindow, nwg_window!( title="Tools"; size=(250, 42) )),

        (SelectToolBtn, nwg_button!(
             parent=ToolWindow;
             text="S";
             position=(5 + 0 * (32 + 5), 5); size=(32, 32);
             font=Some(TextFont))),
        (PointToolBtn, nwg_button!(
             parent=ToolWindow;
             text="P";
             position=(5 + 1 * (32 + 5), 5); size=(32, 32);
             font=Some(TextFont))),
        (LineToolBtn, nwg_button!(
             parent=ToolWindow;
             text="L";
             position=(5 + 2 * (32 + 5), 5); size=(32, 32);
             font=Some(TextFont))),
        (CircleToolBtn, nwg_button!(
             parent=ToolWindow;
             text="C";
             position=(5 + 3 * (32 + 5), 5); size=(32, 32);
             font=Some(TextFont))),
        (ViewportDragToolBtn, nwg_button!(
             parent=ToolWindow;
             text="V";
             position=(5 + 4 * (32 + 5), 5); size=(32, 32);
             font=Some(TextFont)))
    ];
    events: [
        (SelectToolBtn, SelectToolEvent, Event::Click, |_ui,_,_,_| {
          let mut gui_data = GUI_DATA.lock().unwrap();
          *gui_data = Some(Tool::Select);
          drop(gui_data);
        }),
        (PointToolBtn, PointToolEvent, Event::Click, |_ui,_,_,_| {
          let mut gui_data = GUI_DATA.lock().unwrap();
          *gui_data = Some(Tool::Point);
          drop(gui_data);
        }),
        (LineToolBtn, LineToolEvent, Event::Click, |_ui,_,_,_| {
          let mut gui_data = GUI_DATA.lock().unwrap();
          *gui_data = Some(Tool::Line);
          drop(gui_data);
        }),
        (CircleToolBtn, CircleToolEvent, Event::Click, |_ui,_,_,_| {
          let mut gui_data = GUI_DATA.lock().unwrap();
          *gui_data = Some(Tool::Circle);
          drop(gui_data);
        }),
        (ViewportDragToolBtn, ViewportDragToolEvent, Event::Click, |_ui,_,_,_| {
          let mut gui_data = GUI_DATA.lock().unwrap();
          *gui_data = Some(Tool::ViewportDrag);
          drop(gui_data);
        })
    ];
    resources: [
        (TextFont, nwg_font!(family="Arial"; size=17))
    ];
    values: []
);


pub struct GuiSystem {
  init: bool,
}

impl Default for GuiSystem {
  fn default() -> Self {
    Self {
      init: false,
    }
  }
}

impl<'a> System<'a> for GuiSystem {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, ToolChangeEventChannel>,
  );

  fn run(&mut self, (_input_state, mut tool_change_events): Self::SystemData) {
    if !self.init {
      self.init = true;
      spawn(move || {
        start_gui();
      });
      return;
    }
    let mut gui_data = GUI_DATA.lock().unwrap();
    if (*gui_data).is_some() {
      tool_change_events.single_write(ToolChangeEvent((*gui_data).unwrap()));
      *gui_data = None;
    }
    drop(gui_data);
  }
}

fn start_gui() {
  let ui: Ui<AppId> = Ui::new().unwrap();
  setup_ui(&ui).unwrap();
  dispatch_events();
}