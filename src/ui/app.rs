use std::collections::BTreeMap;

pub struct Application {
  pub geometry: Context,
  pub stylesheet: Storage<Style>,
  pub style_map: BTreeMap<Id, Id>,
}