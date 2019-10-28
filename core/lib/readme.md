# Geopad Core Lib

This is the core library of geometry sketchpad. It includes the core components, systems, events and resources to make the application runnable.

There are several kinds of events you can emit to the system:

- `ViewportEvent`. You should emit viewport event when the viewport is moving, scaling, or resizing.
- `HistoryEvent`. You should emit history event to undo or redo.
- `CommandEvent`. You should emit command event whenever you want to select/deselect, hide/unhide, insert, update, remove, or modify geometry components.

There are also some responding event emit from core lib:

- `GeometryEvent`. When a geometry element is inserted, updated, removed, or modified, you will get `GeometryEvent`;
- `MarkerEvent`. When a geometry element is selected/deselected, hidden/unhidden, you will get `MarkerEvent`.