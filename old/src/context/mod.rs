use crate::{
  geometry::{
    line::LineConstruct,
    point::PointConstruct,
  },
  util::Storage
};

pub mod solver;

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
}