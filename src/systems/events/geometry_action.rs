use shrev::{EventChannel, ReaderId};

pub enum GeometryAction {
  // SelectAll,
  // DeselectAll,
  RemoveSelected,
}

pub type GeometryActionChannel = EventChannel<GeometryAction>;

pub type GeometryActionReader = ReaderId<GeometryAction>;