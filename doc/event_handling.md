# Event and Handling Sequence in CoreLib

This file documents how events should happen in the world

## Events

- File Event: Commands asking for "Saving to currently opened file", "Save to a new file", "Load from an existing file"
- Viewport Event: Commands asking for moving/scaling the viewport. That will correspond to resizing and scrolling of the window. Note that file load can also trigger viewport event
- History Event: Commands including "clear", "redo" and "undo"
- Command Event: Commands related to the geometries. "insert", "remove", "update", "select", "hide". There might be more to it because commands come from different places. User input, History, File load, Program input, Animation system and so on.
- Geometry Event: Events emitted from command event involving "insert", "remove" and "update" of geometries
- Marker Event: Events emitted from command event involving "select" and "hide" of geometries

## General flow

```
Event Handlers -> Command Handlers -> Solvers -> Data Managers
```

## Detailed explaination of what's the data flow

1. (**TODO**) The `file_event` should be handled
   1. The file load event will load file, remove all existing geometries and insert all geometries in the file. It will modify the global file resource.
   2. The file save event will save the current world to the file
   3. It will be splitted to multiple `command_event` and `viewport_event` and `history_event`.
      1. For `viewport_event`: since there's viewport information in the file.
      2. For `history_event`: we need to clear the history before loading the file.
      3. Note that all the insertion will be using the `ByHistory` variant in `command_event`, since we don't want the load file process to be recorded by history manager.
2. The `viewport_event`s should be handled
   1. Update `Viewport` resource accordingly
3. The `history_event`s should be handled
   1. They will be splitted to multiple command events
4. The `command_event`s should be handled
   1. `RemoveSystem`, `UpdateSystem`, `InsertSystem`, `HideSystem`, `SelectSystem` will be handling the command events.
   2. They will be further emitted to `geometry_event`s and `marker_event`s. Those events represent that the actions have already been made. Note that there are enums in command event regarding wether the command is made by history. The emitted `geometry_event` and `marker_event` should capture this
5. The `geometry_event`s and `marker_event`s will be handled
   1. `VirtualShapeSolver` will look at symbolic storages and solve for virtual shapes.
      1. It will only look at the updates in `geometry_event`.
   2. `ScreenShapeSolver` will look at virtual shapes and viewport to store all screen shapes.
   3. `SpatialEntityMapManager` will look at `geometry_event`'s insertion/removal/update, `marker_event`'s hide/unhide, and `viewport_event`.
      1. When `viewport_event` happens, it usually means we have to refresh the whole screen.
      2. If not `viewport_event`, insertion/removal will be made when insert/removal/update/hide/unhide happens
      3. It will directly use the result from `ScreenShapeSolver`
   4. `DependencyGraphManager` will look at `geometry_event`'s insertion/removal
   5. `HistoryManager` will look at `geometry_event` and update history.
      1. Note that it will filter out all the events made by history

## Dependency Graph

- (**TODO**) File Event Handler: None
- Viewport Event Handler: File Event Handler
- History Event Handler: File Event Handler
- Command Handlers:
  - Remove system: File Event Handler, History Event Handler;
  - Insert * system: File Event Handler, History Event Handler;
  - Update * system: History Event Handler;
  - Hide system: History Event Handler;
  - Select system: None
  - (**TODO**) Update default style system
- (**THERE SHOULD BE A SYNC HERE**)
- Virtual Shape Solver: All Command Handlers
- Screen Shape Solver: Virtual Shape Solver
- Spatial Entity Map Manager: Screen Shape Solver
- Dependency Graph Manager: All Command Handlers
- History Manager: All Command Handlers