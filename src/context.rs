use crate::{
  geometry::{
    line::LineConstruct,
    point::PointConstruct,
  },
  storage::Storage
};

pub struct Context {
  points: Storage<PointConstruct>,
  lines: Storage<LineConstruct>,
}