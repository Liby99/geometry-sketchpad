use specs::prelude::*;
use nwg::{Event, Ui, dispatch_events};
use std::thread::spawn;
use std::sync::Mutex;
use crate::resources::{InputState, Tool, events::{ToolChangeEventChannel, ToolChangeEvent}};
use shrev::{EventChannel, ReaderId};

enum GuiSystemAction {
  ToolChange(Tool)
}

type GuiSystemActionChannel = EventChannel<GuiSystemAction>;
type GuiSystemActionReader = ReaderId<GuiSystemAction>;

#[cfg(windows)]
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
  static ref GUI_ACTION_CHANNEL : Mutex<GuiSystemActionChannel> = Mutex::new(GuiSystemActionChannel::with_capacity(16));
}

use AppId::*; // Shortcut

nwg_template!(
    head: setup_ui<AppId>,
    controls: [
        (ToolWindow, nwg_window!( title="Tools"; size=(250, 42); resizable=false )),

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
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Select));
        }),
        (PointToolBtn, PointToolEvent, Event::Click, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Point));
        }),
        (LineToolBtn, LineToolEvent, Event::Click, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Line));
        }),
        (CircleToolBtn, CircleToolEvent, Event::Click, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Circle));
        }),
        (ViewportDragToolBtn, ViewportDragToolEvent, Event::Click, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::ViewportDrag));
        })
    ];
    resources: [
        (TextFont, nwg_font!(family="Arial"; size=17))
    ];
    values: []
);


pub struct GuiSystem {
  init: bool,
  gui_action_reader: Option<GuiSystemActionReader>,
}

impl Default for GuiSystem {
  fn default() -> Self {
    Self {
      init: false,
      gui_action_reader: None,
    }
  }
}

impl<'a> System<'a> for GuiSystem {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, ToolChangeEventChannel>,
  );
  
  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.gui_action_reader = Some((*GUI_ACTION_CHANNEL).lock().unwrap().register_reader());
  }
  
  fn run(&mut self, (_input_state, mut tool_change_events): Self::SystemData) {
    if !self.init {
      self.init = true;
      spawn(move || {
        start_gui();
      });
      return;
    }
    if let Some(reader_id) = &mut self.gui_action_reader {
      for event in (*GUI_ACTION_CHANNEL).lock().unwrap().read(reader_id) {
        match event {
          GuiSystemAction::ToolChange(tool) => {
            tool_change_events.single_write(ToolChangeEvent(*tool));
          }
        }
      }
    }
  }
}

fn start_gui() {
  let ui: Ui<AppId> = Ui::new().unwrap();
  setup_ui(&ui).unwrap();
  dispatch_events();
}
