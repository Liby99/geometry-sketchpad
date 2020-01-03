use crate::core_lib::events::*;
use crate::core_lib::math::LineType;
use crate::core_ui::events::{ExitEvent, ExitEventChannel, ToolChangeEvent, ToolChangeEventChannel};
use crate::core_ui::resources::{InputState, Tool};
use nwg::{Event, Ui};
use shrev::{EventChannel, ReaderId};
use specs::prelude::*;
use std::sync::Mutex;

enum GuiSystemAction {
  ToolChange(Tool),
  History(HistoryEvent),
  Command(CommandEvent),
  Resize,
  Exit,
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
  MenuSelect,
  MenuSelectAll,
  MenuSelectDeselect,
  MenuCreate,
  MenuCreateMid,
  MenuCreateParallel,
  MenuCreatePerpendicular,
  MenuDisplay,
  MenuDisplayHide,
  MenuDisplayUnhiddenAll,
  MenuHelp,
  MenuHelpIssue,
  MenuHelpAbout,
  OpenFileDialog,

  // tool buttons
  SelectToolBtn,
  PointToolBtn,
  LineToolBtn,
  LineRayToolBtn,
  LineSegmentToolBtn,
  CircleToolBtn,
  ViewportDragToolBtn,

  // events
  FileOpenEvent,
  FileExitEvent,
  EditUndoEvent,
  EditRedoEvent,
  SelectAllEvent,
  SelectDeselectEvent,
  CreateMidEvent,
  CreateParallelEvent,
  CreatePerpendicularEvent,
  DisplayHideEvent,
  DisplayUnhiddenAllEvent,
  HelpIssueEvent,
  HelpAboutEvent,

  SelectToolEvent,
  PointToolEvent,
  LineToolEvent,
  LineRayToolEvent,
  LineSegmentToolEvent,
  CircleToolEvent,
  ViewportDragToolEvent,

  WindowCloseEvent,
  WindowResizedEvent,
}

lazy_static! {
  static ref GUI_ACTION_CHANNEL: Mutex<GuiSystemActionChannel> = Mutex::new(GuiSystemActionChannel::with_capacity(16));
}

use AppId::*;

const WINDOW_WIDTH: i32 = 960;
const WINDOW_HEIGHT: i32 = 720;
const WINDOW_TOOLBAR_HEIGHT: i32 = 42;

nwg_template!(
  head: setup_ui<AppId>,
  controls: [
    (MainWindow, nwg_window!( title="Rust Geometry Sketchpad"; size=(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32); resizable=true )),
    (MenuFile, nwg_menu!(
          parent=MainWindow;
          text="File"
    )),
        (MenuFileOpen, nwg_menuitem!(
              parent=MenuFile;
              text="&Open...\tCtrl+O";
              disabled=true
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

    (MenuSelect, nwg_menu!(
          parent=MainWindow;
          text="Select"
    )),
        (MenuSelectAll, nwg_menuitem!(
              parent=MenuSelect;
              text="&Select All\tCtrl+A"
        )),
        (MenuSelectDeselect, nwg_menuitem!(
              parent=MenuSelect;
              text="&Deselect\tCtrl+D"
        )),

    (MenuCreate, nwg_menu!(
          parent=MainWindow;
          text="Create"
    )),
        (MenuCreateMid, nwg_menuitem!(
              parent=MenuCreate;
              text="&Mid Point\tCtrl+M"
        )),
        (MenuCreateParallel, nwg_menuitem!(
              parent=MenuCreate;
              text="&Parallel Line\tCtrl+Shift+_"
        )),
        (MenuCreatePerpendicular, nwg_menuitem!(
              parent=MenuCreate;
              text="&Perpendicular Line\tCtrl+Shift+\\"
        )),

    (MenuDisplay, nwg_menu!(
          parent=MainWindow;
          text="Display"
    )),
        (MenuDisplayHide, nwg_menuitem!(
              parent=MenuDisplay;
              text="&Hide Selection\tCtrl+H"
        )),
        (MenuDisplayUnhiddenAll, nwg_menuitem!(
              parent=MenuDisplay;
              text="&Unhidden All\tCtrl+Shift+H"
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
          text="select";
          position=(5 + 0 * (32 + 5), 5); size=(32, 32))),
    (PointToolBtn, nwg_button!(
          parent=MainWindow;
          text="point";
          position=(5 + 1 * (32 + 5), 5); size=(32, 32))),
    (LineToolBtn, nwg_button!(
          parent=MainWindow;
          text="line";
          position=(5 + 2 * (32 + 5), 5); size=(32, 32))),
    (LineRayToolBtn, nwg_button!(
          parent=MainWindow;
          text="line.ray";
          position=(5 + 3 * (32 + 5), 5); size=(32, 32))),
    (LineSegmentToolBtn, nwg_button!(
          parent=MainWindow;
          text="line.segment";
          position=(5 + 4 * (32 + 5), 5); size=(32, 32))),
    (CircleToolBtn, nwg_button!(
          parent=MainWindow;
          text="circle";
          position=(5 + 5 * (32 + 5), 5); size=(32, 32))),
    (ViewportDragToolBtn, nwg_button!(
          parent=MainWindow;
          text="viewport.drag";
          position=(5 + 6 * (32 + 5), 5); size=(32, 32)))
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

    (MenuFileExit, FileExitEvent, Event::Triggered, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Exit);
    }),
    (MenuEditUndo, EditUndoEvent, Event::Triggered, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::History(HistoryEvent::Undo));
    }),
    (MenuEditRedo, EditRedoEvent, Event::Triggered, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::History(HistoryEvent::Redo));
    }),


    (MenuSelectAll, SelectAllEvent, Event::Triggered, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Command(CommandEvent::Select(SelectEvent::SelectAll)));
    }),
    (MenuSelectDeselect, SelectDeselectEvent, Event::Triggered, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Command(CommandEvent::Select(SelectEvent::DeselectAll)));
    }),
    (MenuCreateMid, CreateMidEvent, Event::Triggered, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Command(CommandEvent::PointInsert(InsertPointEvent::InsertMidPointFromSelection)));
    }),
    (MenuCreateParallel, CreateParallelEvent, Event::Triggered, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Command(CommandEvent::LineInsert(InsertLineEvent::InsertParallelFromSelection)));
    }),
    (MenuCreatePerpendicular, CreatePerpendicularEvent, Event::Triggered, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Command(CommandEvent::LineInsert(InsertLineEvent::InsertPerpendicularFromSelection)));
    }),
    (MenuDisplayHide, DisplayHideEvent, Event::Triggered, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Command(CommandEvent::Hide(HideEvent::HideSelected)));
    }),
    (MenuDisplayUnhiddenAll, DisplayUnhiddenAllEvent, Event::Triggered, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Command(CommandEvent::Hide(HideEvent::UnhideAll)));
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
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Line(LineType::Straight)));
    }),
    (LineRayToolBtn, LineRayToolEvent, Event::Click, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Line(LineType::Ray)));
    }),
    (LineSegmentToolBtn, LineSegmentToolEvent, Event::Click, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Line(LineType::Segment)));
    }),
    (CircleToolBtn, CircleToolEvent, Event::Click, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Circle));
    }),
    (ViewportDragToolBtn, ViewportDragToolEvent, Event::Click, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::ToolChange(Tool::Viewport));
    }),


    (MainWindow, WindowCloseEvent, Event::Closed, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Exit);
    }),
    (MainWindow, WindowResizedEvent, Event::Resized, |_ui,_,_,_| {
      (*GUI_ACTION_CHANNEL).lock().unwrap().single_write(GuiSystemAction::Resize);
    })
  ];
  resources: [];
  values: []
);

pub struct GuiSystem {
  gui_action_reader: Option<GuiSystemActionReader>,
  ui: Option<Ui<AppId>>,
  handle: winapi::HWND,
  piston: winapi::HWND,
}

impl Default for GuiSystem {
  fn default() -> Self {
    let ui = Ui::new().unwrap();
    setup_ui(&ui).unwrap();

    let mut handle: winapi::HWND = 0 as winapi::HWND;
    let mut handle_piston: winapi::HWND = 0 as winapi::HWND;
    unsafe {
      use std::ffi::OsStr;
      use std::os::windows::ffi::OsStrExt;
      use user32::{FindWindowW, GetClientRect, SetParent, SetWindowLongPtrW, SetWindowPos};
      use winapi::{
        GWL_STYLE, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOOWNERZORDER, SWP_NOZORDER, WS_POPUP, WS_VISIBLE,
      };
      if let Ok(window) = ui.get::<nwg::Window>(&MainWindow) {
        use crate::nwg::custom::*;
        if let AnyHandle::HWND(h) = window.handle() {
          handle = h;
        } else {
          panic!()
        }
      } else {
        panic!()
      }
      handle_piston = FindWindowW(
        std::ptr::null_mut(),
        OsStr::new("Geometry Sketchpad")
          .encode_wide()
          .chain(Some(0))
          .collect::<Vec<_>>()
          .as_ptr(),
      );
      SetParent(handle_piston, handle);
      SetWindowLongPtrW(handle_piston, GWL_STYLE, (WS_POPUP | WS_VISIBLE) as i64);
      let mut rect: winapi::RECT = std::mem::zeroed();
      if GetClientRect(handle, &mut rect) != 0 {
        let width = rect.right - rect.left - 1;
        let height = rect.bottom - rect.top - 1;
        SetWindowPos(
          handle_piston,
          0 as winapi::HWND,
          0,
          WINDOW_TOOLBAR_HEIGHT,
          width,
          height - WINDOW_TOOLBAR_HEIGHT,
          SWP_NOACTIVATE | SWP_NOZORDER | SWP_NOOWNERZORDER | SWP_FRAMECHANGED,
        );
      }
    }

    Self {
      gui_action_reader: None,
      ui: Some(ui),
      handle: handle,
      piston: handle_piston,
    }
  }
}

impl<'a> System<'a> for GuiSystem {
  type SystemData = (
    Read<'a, InputState>,
    Write<'a, ToolChangeEventChannel>,
    Write<'a, ExitEventChannel>,
    Write<'a, HistoryEventChannel>,
    Write<'a, CommandEventChannel>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.gui_action_reader = Some((*GUI_ACTION_CHANNEL).lock().unwrap().register_reader());
  }

  fn run(
    &mut self,
    (_input_state, mut tool_change_events, mut exit_events, mut history_events, mut command_events): Self::SystemData,
  ) {
    unsafe {
      use user32::{
        BringWindowToTop, DispatchMessageW, GetCursorPos, PeekMessageW, SendMessageW, TranslateMessage, WindowFromPoint,
      };
      let mut msg: winapi::winuser::MSG = std::mem::zeroed();
      if PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, 1) != 0 {
        use winapi::{WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP};
        if msg.message == WM_KEYDOWN
          || msg.message == WM_KEYUP
          || msg.message == WM_SYSKEYDOWN
          || msg.message == WM_SYSKEYUP
        {
          /* Forward keyboard message to piston window */
          SendMessageW(self.piston, msg.message, msg.wParam, msg.lParam);
        }
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
      }
      let mut cursor: winapi::POINT = std::mem::zeroed();
      GetCursorPos(&mut cursor);
      if WindowFromPoint(cursor) == self.piston {
        BringWindowToTop(self.piston);
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
            history_events.single_write(*action);
          }
          GuiSystemAction::Resize => unsafe {
            use user32::{GetClientRect, SetWindowPos};
            use winapi::{SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOOWNERZORDER, SWP_NOZORDER};
            let mut rect: winapi::RECT = std::mem::zeroed();
            if GetClientRect(self.handle, &mut rect) != 0 {
              let width = rect.right - rect.left - 1;
              let height = rect.bottom - rect.top - 1;
              SetWindowPos(
                self.piston,
                0 as winapi::HWND,
                0,
                WINDOW_TOOLBAR_HEIGHT,
                width,
                height - WINDOW_TOOLBAR_HEIGHT,
                SWP_NOACTIVATE | SWP_NOZORDER | SWP_NOOWNERZORDER | SWP_FRAMECHANGED,
              );
            }
          },
          GuiSystemAction::Command(event) => {
            command_events.single_write(*event);
          }
        }
      }
    }
  }
}
