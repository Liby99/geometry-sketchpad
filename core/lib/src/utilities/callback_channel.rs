use std::hash::Hash;
use std::collections::HashMap;
use shrev::*;

pub struct EmittedEvent<Id, Info> where
  Id: 'static + Hash + Eq + Send + Sync + Copy,
  Info: 'static + Send + Sync + Copy,
{
  id: Id,
  info: Info,
}

impl<Id, Info> EmittedEvent<Id, Info> where
  Id: 'static + Hash + Eq + Send + Sync + Copy,
  Info: 'static + Send + Sync + Copy,
{
  pub fn id(&self) -> Id {
    self.id
  }

  pub fn info(&self) -> Info {
    self.info
  }
}

pub struct CallbackEvent<Id, Info> where
  Id: 'static + Hash + Eq + Send + Sync + Copy,
  Info: 'static + Send + Sync + Copy,
{
  id: Id,
  info: Info,
}

pub struct ClosureStorage<Id, Info, ClosureData> where
  Id: 'static + Hash + Eq + Send + Sync + Copy,
  Info: 'static + Send + Sync + Copy,
{
  reader: ReaderId<CallbackEvent<Id, Info>>,
  storage: HashMap<Id, ClosureData>,
}

pub trait IdAllocator<Id: Eq + Copy> {
  fn allocate(&mut self) -> Id;
  fn recycle(&mut self, id: Id);
}

pub struct U32IdAllocator {
  maximum: u32,
  recycled: Vec<u32>,
}

impl IdAllocator<u32> for U32IdAllocator {
  fn allocate(&mut self) -> u32 {
    if let Some(rec) = self.recycled.pop() {
      rec
    } else {
      self.maximum += 1;
      self.maximum
    }
  }

  fn recycle(&mut self, id: u32) {
    self.recycled.push(id);
  }
}

pub struct CallbackChannel<Id, Allocator, EmitInfo, CallbackInfo> where
  Id: 'static + Hash + Eq + Send + Sync + Copy,
  Allocator: IdAllocator<Id>,
  EmitInfo: 'static + Event + Copy,
  CallbackInfo: 'static + Event + Copy,
{
  allocator: Allocator,
  emit_channel: EventChannel<EmittedEvent<Id, EmitInfo>>,
  callback_channel: EventChannel<CallbackEvent<Id, CallbackInfo>>,
}

impl<Id, Allocator, EmitInfo, CallbackInfo> CallbackChannel<Id, Allocator, EmitInfo, CallbackInfo> where
  Id: 'static + Hash + Eq + Send + Sync + Copy,
  Allocator: IdAllocator<Id>,
  EmitInfo: 'static + Event + Copy,
  CallbackInfo: 'static + Event + Copy,
{
  pub fn register_closure_storage<ClosureInfo>(&mut self) -> ClosureStorage<Id, CallbackInfo, ClosureInfo> where
    ClosureInfo: Copy,
  {
    let reader = self.callback_channel.register_reader();
    ClosureStorage { reader, storage: HashMap::new() }
  }

  pub fn register_event_reader(&mut self) -> ReaderId<EmittedEvent<Id, EmitInfo>> {
    self.emit_channel.register_reader()
  }

  pub fn emit<ClosureInfo>(
    &mut self,
    clos_storage: &mut ClosureStorage<Id, CallbackInfo, ClosureInfo>,
    clos_info: ClosureInfo,
    emit_info: EmitInfo
  ) where
    ClosureInfo: Copy,
  {
    let id = self.allocator.allocate();
    clos_storage.storage.insert(id, clos_info);
    let emit_event = EmittedEvent { id, info: emit_info };
    self.emit_channel.single_write(emit_event);
  }

  pub fn read_emitted(&self, reader: &mut ReaderId<EmittedEvent<Id, EmitInfo>>) -> EventIterator<EmittedEvent<Id, EmitInfo>> {
    self.emit_channel.read(reader)
  }

  pub fn callback(&mut self, id: Id, cb_info: CallbackInfo) {
    let callback_event = CallbackEvent { id, info: cb_info };
    self.callback_channel.single_write(callback_event);
  }

  pub fn read_callback<ClosureInfo>(
    &self,
    closure_storage: &mut ClosureStorage<Id, CallbackInfo, ClosureInfo>,
  ) -> Vec<(CallbackInfo, ClosureInfo)> where
    ClosureInfo: Copy,
  {
    self.callback_channel.read(&mut closure_storage.reader)
      .filter_map(|CallbackEvent { id, info }| {
        if let Some(closure) = closure_storage.storage.get(&id) {
          Some((*info, *closure))
        } else {
          None
        }
      }).collect::<Vec<_>>()
  }

  // Add free listener functionality
}