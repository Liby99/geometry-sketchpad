use crate::{
  util::{Id, Storage},
  geometry::{LineConstruct, PointConstruct},
};

pub struct Context {
  pub points: Storage<PointConstruct>,
  pub lines: Storage<LineConstruct>,
}

impl Context {
  pub fn new() -> Context {
    Context {
      points: Storage::new(),
      lines: Storage::new(),
    }
  }

  pub fn add_point(&mut self, p: PointConstruct) -> Id {
    self.points.add(p)
  }

  pub fn add_line(&mut self, l: LineConstruct) -> Id {
    self.lines.add(l)
  }
}