use std::time::SystemTime;
use std::collections::HashMap;
use core_lib::{math::*, utilities::*};

pub struct InputState {
  pub mouse_left_button: ActiveState,
  pub mouse_right_button: ActiveState,

  pub is_mouse_left_button_dragging: bool,
  pub mouse_left_button_last_pressed: Option<SystemTime>,

  pub mouse_abs_pos: ScreenPosition,
  pub mouse_rel_movement: ScreenPosition,
  pub rel_scroll: Vector2,
  pub in_focus: ActiveState,
  pub keyboard: Keyboard,
}

impl Default for InputState {
  fn default() -> Self {
    Self {
      mouse_left_button: ActiveState::default(),
      mouse_right_button: ActiveState::default(),
      is_mouse_left_button_dragging: false,
      mouse_left_button_last_pressed: None,
      mouse_abs_pos: vec2![0., 0.].into(),
      mouse_rel_movement: vec2![0., 0.].into(),
      in_focus: ActiveState::default(),
      rel_scroll: vec2![0., 0.],
      keyboard: Keyboard::default(),
    }
  }
}

impl InputState {
  pub fn reset_relative_data(&mut self) {
    self.mouse_left_button.reset_relative_data();
    self.mouse_right_button.reset_relative_data();
    self.mouse_rel_movement = vec2![0., 0.].into();
    self.in_focus.reset_relative_data();
    self.rel_scroll = vec2![0., 0.];
    self.keyboard.reset_relative_data();
  }
}

pub struct ActiveState {
  pressed: bool,
  just_changed: bool,
}

impl Default for ActiveState {
  fn default() -> Self {
    Self { pressed: false, just_changed: false }
  }
}

impl ActiveState {
  pub fn new(pressed: bool, just_changed: bool) -> Self {
    Self { pressed, just_changed }
  }

  pub fn set(&mut self, next: bool) {
    if self.pressed != next {
      self.pressed = next;
      self.just_changed = true;
    }
  }

  pub fn is_activated(&self) -> bool {
    self.pressed
  }

  pub fn just_activated(&self) -> bool {
    self.pressed && self.just_changed
  }

  pub fn just_deactivated(&self) -> bool {
    !self.pressed && self.just_changed
  }

  pub fn reset_relative_data(&mut self) {
    self.just_changed = false;
  }
}

pub struct Keyboard {
  keys: HashMap<Key, ActiveState>,
}

impl Default for Keyboard {
  fn default() -> Self {
    Self { keys: HashMap::new() }
  }
}

impl Keyboard {
  pub fn set(&mut self, key: Key, pressed: bool) {
    match self.keys.get_mut(&key) {
      Some(state) => state.set(pressed),
      None => if pressed { self.keys.insert(key, ActiveState::new(true, true)); }
    }
  }

  pub fn is_activated(&self, key: Key) -> bool {
    match self.keys.get(&key) {
      Some(state) => state.pressed,
      None => false,
    }
  }

  pub fn just_activated(&self, key: Key) -> bool {
    match self.keys.get(&key) {
      Some(state) => state.just_activated(),
      None => false,
    }
  }

  pub fn just_deactivated(&self, key: Key) -> bool {
    match self.keys.get(&key) {
      Some(state) => state.just_deactivated(),
      None => false,
    }
  }

  pub fn is_shift_activated(&self) -> bool {
    self.is_activated(Key::LShift) || self.is_activated(Key::RShift)
  }

  pub fn is_command_activated(&self) -> bool {
    if cfg!(target_os = "macos") {
      self.is_activated(Key::LCommand) || self.is_activated(Key::RCommand)
    } else {
      self.is_activated(Key::LCtrl) || self.is_activated(Key::RCtrl)
    }
  }

  pub fn reset_relative_data(&mut self) {
    for (_, state) in self.keys.iter_mut() {
      state.reset_relative_data();
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
  Unknown = 0x00,

  // Custom keys
  LCommand,
  RCommand,

  // All the base keys
  Backspace = 0x08,
  Tab = 0x09,
  Return = 0x0D,
  Escape = 0x1B,
  Space = 0x20,
  Exclaim = 0x21,
  Quotedbl = 0x22,
  Hash = 0x23,
  Dollar = 0x24,
  Percent = 0x25,
  Ampersand = 0x26,
  Quote = 0x27,
  LeftParen = 0x28,
  RightParen = 0x29,
  Asterisk = 0x2A,
  Plus = 0x2B,
  Comma = 0x2C,
  Minus = 0x2D,
  Period = 0x2E,
  Slash = 0x2F,
  D0 = 0x30,
  D1 = 0x31,
  D2 = 0x32,
  D3 = 0x33,
  D4 = 0x34,
  D5 = 0x35,
  D6 = 0x36,
  D7 = 0x37,
  D8 = 0x38,
  D9 = 0x39,
  Colon = 0x3A,
  Semicolon = 0x3B,
  Less = 0x3C,
  Equals = 0x3D,
  Greater = 0x3E,
  Question = 0x3F,
  At = 0x40,
  LeftBracket = 0x5B,
  Backslash = 0x5C,
  RightBracket = 0x5D,
  Caret = 0x5E,
  Underscore = 0x5F,
  Backquote = 0x60,
  A = 0x61,
  B = 0x62,
  C = 0x63,
  D = 0x64,
  E = 0x65,
  F = 0x66,
  G = 0x67,
  H = 0x68,
  I = 0x69,
  J = 0x6A,
  K = 0x6B,
  L = 0x6C,
  M = 0x6D,
  N = 0x6E,
  O = 0x6F,
  P = 0x70,
  Q = 0x71,
  R = 0x72,
  S = 0x73,
  T = 0x74,
  U = 0x75,
  V = 0x76,
  W = 0x77,
  X = 0x78,
  Y = 0x79,
  Z = 0x7A,
  Delete = 0x7F,
  CapsLock = 0x40000039,
  F1 = 0x4000003A,
  F2 = 0x4000003B,
  F3 = 0x4000003C,
  F4 = 0x4000003D,
  F5 = 0x4000003E,
  F6 = 0x4000003F,
  F7 = 0x40000040,
  F8 = 0x40000041,
  F9 = 0x40000042,
  F10 = 0x40000043,
  F11 = 0x40000044,
  F12 = 0x40000045,
  PrintScreen = 0x40000046,
  ScrollLock = 0x40000047,
  Pause = 0x40000048,
  Insert = 0x40000049,
  Home = 0x4000004A,
  PageUp = 0x4000004B,
  End = 0x4000004D,
  PageDown = 0x4000004E,
  Right = 0x4000004F,
  Left = 0x40000050,
  Down = 0x40000051,
  Up = 0x40000052,
  NumLockClear = 0x40000053,
  NumPadDivide = 0x40000054,
  NumPadMultiply = 0x40000055,
  NumPadMinus = 0x40000056,
  NumPadPlus = 0x40000057,
  NumPadEnter = 0x40000058,
  NumPad1 = 0x40000059,
  NumPad2 = 0x4000005A,
  NumPad3 = 0x4000005B,
  NumPad4 = 0x4000005C,
  NumPad5 = 0x4000005D,
  NumPad6 = 0x4000005E,
  NumPad7 = 0x4000005F,
  NumPad8 = 0x40000060,
  NumPad9 = 0x40000061,
  NumPad0 = 0x40000062,
  NumPadPeriod = 0x40000063,
  Application = 0x40000065,
  Power = 0x40000066,
  NumPadEquals = 0x40000067,
  F13 = 0x40000068,
  F14 = 0x40000069,
  F15 = 0x4000006A,
  F16 = 0x4000006B,
  F17 = 0x4000006C,
  F18 = 0x4000006D,
  F19 = 0x4000006E,
  F20 = 0x4000006F,
  F21 = 0x40000070,
  F22 = 0x40000071,
  F23 = 0x40000072,
  F24 = 0x40000073,
  Execute = 0x40000074,
  Help = 0x40000075,
  Menu = 0x40000076,
  Select = 0x40000077,
  Stop = 0x40000078,
  Again = 0x40000079,
  Undo = 0x4000007A,
  Cut = 0x4000007B,
  Copy = 0x4000007C,
  Paste = 0x4000007D,
  Find = 0x4000007E,
  Mute = 0x4000007F,
  VolumeUp = 0x40000080,
  VolumeDown = 0x40000081,
  NumPadComma = 0x40000085,
  NumPadEqualsAS400 = 0x40000086,
  AltErase = 0x40000099,
  Sysreq = 0x4000009A,
  Cancel = 0x4000009B,
  Clear = 0x4000009C,
  Prior = 0x4000009D,
  Return2 = 0x4000009E,
  Separator = 0x4000009F,
  Out = 0x400000A0,
  Oper = 0x400000A1,
  ClearAgain = 0x400000A2,
  CrSel = 0x400000A3,
  ExSel = 0x400000A4,
  NumPad00 = 0x400000B0,
  NumPad000 = 0x400000B1,
  ThousandsSeparator = 0x400000B2,
  DecimalSeparator = 0x400000B3,
  CurrencyUnit = 0x400000B4,
  CurrencySubUnit = 0x400000B5,
  NumPadLeftParen = 0x400000B6,
  NumPadRightParen = 0x400000B7,
  NumPadLeftBrace = 0x400000B8,
  NumPadRightBrace = 0x400000B9,
  NumPadTab = 0x400000BA,
  NumPadBackspace = 0x400000BB,
  NumPadA = 0x400000BC,
  NumPadB = 0x400000BD,
  NumPadC = 0x400000BE,
  NumPadD = 0x400000BF,
  NumPadE = 0x400000C0,
  NumPadF = 0x400000C1,
  NumPadXor = 0x400000C2,
  NumPadPower = 0x400000C3,
  NumPadPercent = 0x400000C4,
  NumPadLess = 0x400000C5,
  NumPadGreater = 0x400000C6,
  NumPadAmpersand = 0x400000C7,
  NumPadDblAmpersand = 0x400000C8,
  NumPadVerticalBar = 0x400000C9,
  NumPadDblVerticalBar = 0x400000CA,
  NumPadColon = 0x400000CB,
  NumPadHash = 0x400000CC,
  NumPadSpace = 0x400000CD,
  NumPadAt = 0x400000CE,
  NumPadExclam = 0x400000CF,
  NumPadMemStore = 0x400000D0,
  NumPadMemRecall = 0x400000D1,
  NumPadMemClear = 0x400000D2,
  NumPadMemAdd = 0x400000D3,
  NumPadMemSubtract = 0x400000D4,
  NumPadMemMultiply = 0x400000D5,
  NumPadMemDivide = 0x400000D6,
  NumPadPlusMinus = 0x400000D7,
  NumPadClear = 0x400000D8,
  NumPadClearEntry = 0x400000D9,
  NumPadBinary = 0x400000DA,
  NumPadOctal = 0x400000DB,
  NumPadDecimal = 0x400000DC,
  NumPadHexadecimal = 0x400000DD,
  LCtrl = 0x400000E0,
  LShift = 0x400000E1,
  LAlt = 0x400000E2,
  LGui = 0x400000E3,
  RCtrl = 0x400000E4,
  RShift = 0x400000E5,
  RAlt = 0x400000E6,
  RGui = 0x400000E7,
  Mode = 0x40000101,
  AudioNext = 0x40000102,
  AudioPrev = 0x40000103,
  AudioStop = 0x40000104,
  AudioPlay = 0x40000105,
  AudioMute = 0x40000106,
  MediaSelect = 0x40000107,
  Www = 0x40000108,
  Mail = 0x40000109,
  Calculator = 0x4000010A,
  Computer = 0x4000010B,
  AcSearch = 0x4000010C,
  AcHome = 0x4000010D,
  AcBack = 0x4000010E,
  AcForward = 0x4000010F,
  AcStop = 0x40000110,
  AcRefresh = 0x40000111,
  AcBookmarks = 0x40000112,
  BrightnessDown = 0x40000113,
  BrightnessUp = 0x40000114,
  DisplaySwitch = 0x40000115,
  KbdIllumToggle = 0x40000116,
  KbdIllumDown = 0x40000117,
  KbdIllumUp = 0x40000118,
  Eject = 0x40000119,
  Sleep = 0x4000011A,
}