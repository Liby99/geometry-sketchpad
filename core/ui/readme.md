# Geopad Core UI

This library provides an abstraction of interaction interface, and a bunch of standard interaction scheme with Geopad. This UI library wraps around Core Library and provides additional event systems for user to interact with this system.

To interact with Core UI, you need

- `InputState`: Feed correct key down, key up, mouse position, mouse button states to this `InputState` struct
- `MouseEventChannel`: Feed correct mouse events (mouse drag, click) to this channel

Additionally, you can also provide these events to mutate system states

- `ToolChangeEvent`: Emit tool change event to this channel to change the tool
- `ExitEvent`: Emit exit event to terminate

Of course you can continue to provide events to the channels in Core Lib:

- `ViewportEvent`
- `HistoryEvent`
- `CommandEvent`

The events that you are going to receive back is still `GeometryEvent` and `MarkerEvent`.