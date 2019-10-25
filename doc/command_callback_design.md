# Command callback design

When shooting a command, you can specify whether you want to have a callback.

This event will be then marked with a unique `EventId`, being allocated from somewhere. The event emitter should also store this `EventId`, maybe along with some additional information, namely `Closure`.

The event emitter should be then listening to the callback events, and should be handling this `EventId`. The process will be, when getting an event with an `EventId` being presented in this local storage, one will get three things: `EventId`, `Closure`, and the `CallbackInformation`.

Once the callback is being handled, the `EventId` should be emptied for future use. The user has to specify when to drop the `EventId`

## Things that occur in the process

Globally:
- `EventId` allocator
- Callback Event Channel (Maybe extend the `EventChannel` to `EventCallbackChannel<EventInfoType, CallbackInfoType>`)

Locally:
- `EventId` and `Closure` storage

``` rust
// Event listener
for event in callback_channel.read_event(reader) {
  match event.get() {
    EventData(data) => {
      // process event
      callback_channel.write_callback(event, callback_data);
    }
  }
}
```

``` rust
// When writting event
closures.add(callback_channel.write_event(event_data), closure_data);

// When listening callback
for (closure_data, callback_info) in callback_channel.read_callback(closures) {
  // Handle callback
}
```