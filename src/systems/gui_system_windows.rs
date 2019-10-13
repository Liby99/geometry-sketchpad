use specs::prelude::*;
use nwg::{Event, Ui};
use std::sync::Mutex;
use crate::resources::{InputState, Tool, LineTool, events::*};
use shrev::{EventChannel, ReaderId};

enum GuiSystemAction {
  ToolChange(Tool),
  Exit,
  History(HistoryAction),
}

type GuiSystemActionChannel = EventChannel<GuiSystemAction>;
type GuiSystemActionReader = ReaderId<GuiSystemAction>;

#[cfg(windows)]
#[derive(Debug, Clone, Hash)]
pub enum AppId {

  MainWindow,

  // menu
  MenuFile,
    MenuFileOpen,
    MenuFileSave,
    MenuFileExit,
  MenuEdit,
    MenuEditUndo,
    MenuEditRedo,
  MenuHelp,
    MenuHelpIssue,
    MenuHelpAbout,
  OpenFileDialog,

  // tool buttons
  SelectToolBtn,
  PointToolBtn,
  LineToolBtn,
  CircleToolBtn,
  ViewportDragToolBtn,

  // events
  FileOpenEvent,
  FileExitEvent,
  EditUndoEvent,
  EditRedoEvent,
  
  HelpIssueEvent,
  HelpAboutEvent,

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
        (MainWindow, nwg_window!( title="Rust Geometry Sketchpad"; size=(250, 60); resizable=false )),
        (MenuFile, nwg_menu!(
             parent=MainWindow;
             text="File"
        )),
            (MenuFileOpen, nwg_menuitem!(
                  parent=MenuFile;
                  text="&Open...\tCtrl+O"
            )),
            (MenuFileSave, nwg_menuitem!(
                  parent=MenuFile;
                  text="&Save\tCtrl+S";
                  disabled=true
            )),
            (MenuFileExit, nwg_menuitem!(
                  parent=MenuFile;
                  text="&Exit\tCtrl+Q/W"
            )),
        (MenuEdit, nwg_menu!(
             parent=MainWindow;
             text="Edit"
        )),
            (MenuEditUndo, nwg_menuitem!(
                  parent=MenuEdit;
                  text="&Undo\tCtrl+Z"
            )),
            (MenuEditRedo, nwg_menuitem!(
                  parent=MenuEdit;
                  text="&Redo\tCtrl+Shift+Z"
            )),
        (MenuHelp, nwg_menu!(
             parent=MainWindow;
             text="Help"
        )),
            (MenuHelpIssue, nwg_menuitem!(
                  parent=MenuHelp;
                  text="Report an issue"
            )),
            (MenuHelpAbout, nwg_menuitem!(
                  parent=MenuHelp;
                  text="About"
            )),
        (OpenFileDialog, nwg_filedialog!(
             parent=Some(MainWindow);
             action=nwg::constants::FileDialogAction::Open;
             title="Open...";
             multiselect=false;
             filters=Some("Rust Geometry Sketchpad File(*.rgsp)"))),
        (SelectToolBtn, nwg_button!(
             parent=MainWindow;
             text="S";
             position=(5 + 0 * (32 + 5), 5); size=(32, 32);
             font=Some(TextFont))),
        (PointToolBtn, nwg_button!(
             parent=MainWindow;
             text="P";
             position=(5 + 1 * (32 + 5), 5); size=(32, 32);
             font=Some(TextFont))),
        (LineToolBtn, nwg_button!(
             parent=MainWindow;
             text="L";
             position=(5 + 2 * (32 + 5), 5); size=(32, 32);
             font=Some(TextFont))),
        (CircleToolBtn, nwg_button!(
             parent=MainWindow;
             text="C";
             position=(5 + 3 * (32 + 5), 5); size=(32, 32);
             font=Some(TextFont))),
        (ViewportDragToolBtn, nwg_button!(
             parent=MainWindow;
             text="V";
             position=(5 + 4 * (32 + 5), 5); size=(32, 32);
             font=Some(TextFont)))
    ];
    events: [
        (MenuFileOpen, FileOpenEvent, Event::Triggered, |ui,_,_,_| {
          if let Ok(file_dialog) = ui.get::<nwg::FileDialog>(&OpenFileDialog) {
            if file_dialog.run() {
              if let Ok(filename) = file_dialog.get_selected_item() {
                println!("File selected ok {}", filename);
              } else {
                panic!()
              }
            } else {
              println!("File open failed");
            }
          } else {
            panic!()
          }
        }),
        (MenuHelpIssue, HelpIssueEvent, Event::Triggered, |_ui,_,_,_| {
          let _ = open::that("https://github.com/Liby99/geometry-sketchpad/issues/new");
        }),
        (MenuHelpAbout, HelpAboutEvent, Event::Triggered, |_ui,_,_,_| {
          let _ = open::that("https://github.com/Liby99/geometry-sketchpad");
        }),
        (SelectToolBtn, SelectToolEvent, Event::Click, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Select));
        }),
        (PointToolBtn, PointToolEvent, Event::Click, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Point));
        }),
        (LineToolBtn, LineToolEvent, Event::Click, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Line(LineTool::Line)));
        }),
        (CircleToolBtn, CircleToolEvent, Event::Click, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Circle));
        }),
        (ViewportDragToolBtn, ViewportDragToolEvent, Event::Click, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::ViewportDrag));
        }),
        
        (MenuFileExit, FileExitEvent, Event::Triggered, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Exit);
        }),
        (MenuEditUndo, EditUndoEvent, Event::Triggered, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::History(HistoryAction::Undo));
        }),
        (MenuEditRedo, EditRedoEvent, Event::Triggered, |_ui,_,_,_| {
          (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::History(HistoryAction::Redo));
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
  ui: Option<Ui<AppId>>,
}

impl Default for GuiSystem {
  fn default() -> Self {
    Self {
      init: false,
      gui_action_reader: None,
      ui: None,
    }
  }
}

impl<'a> System<'a> for GuiSystem {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, ToolChangeEventChannel>,
    Write<'a, ExitEventChannel>,
    Write<'a, HistoryActionChannel>,
  );
  
  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.gui_action_reader = Some((*GUI_ACTION_CHANNEL).lock().unwrap().register_reader());
  }
  
  fn run(&mut self, (_input_state, mut tool_change_events, mut exit_events, mut history_action_channel): Self::SystemData) {

    if !self.init {
      self.init = true;
      let ui = Ui::new().unwrap();
      setup_ui(&ui).unwrap();
      self.ui = Some(ui);
      return;
    }

    use user32::{PeekMessageW, TranslateMessage, DispatchMessageW};
    unsafe {
      let mut msg: winapi::winuser::MSG = std::mem::uninitialized();
      if PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, 1) != 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
      }
    }
    if let Some(reader_id) = &mut self.gui_action_reader {
      for event in (*GUI_ACTION_CHANNEL).lock().unwrap().read(reader_id) {
        match event {
          GuiSystemAction::ToolChange(tool) => {
            tool_change_events.single_write(ToolChangeEvent(*tool));
          }
          GuiSystemAction::Exit => {
            exit_events.single_write(ExitEvent);
          }
          GuiSystemAction::History(action) => {
            history_action_channel.single_write(*action);
          }
        }
      }
    }
  }
}
