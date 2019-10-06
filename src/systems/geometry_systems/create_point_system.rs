use std::mem::drop;
use specs::prelude::*;
use shrev::EventChannel;
use crate::{
  utilities::Color,
  resources::{
    geometry::{MaybeSnapPoint, SnapPoint, SnapPointType, LastActivePoint},
    events::{
      ToolChangeEvent, ToolChangeEventChannel, ToolChangeEventReader,
      MouseEvent, MouseEventChannel, MouseEventReader,
      SketchEvent, Geometry, SketchEventChannel
    },
  },
  components::{SymbolicPoint, Point, PointStyle, Selected},
};

pub struct CreatePointSystem {
  tool_change_event_reader: Option<ToolChangeEventReader>,
  mouse_event_reader: Option<MouseEventReader>,
}

impl Default for CreatePointSystem {
  fn default() -> Self {
    Self {
      tool_change_event_reader: None,
      mouse_event_reader: None,
    }
  }
}

impl<'a> System<'a> for CreatePointSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, ToolChangeEventChannel>,
    Read<'a, MaybeSnapPoint>,
    Write<'a, MouseEventChannel>,
    Write<'a, SketchEventChannel>,
    Write<'a, EventChannel<LastActivePoint>>,
    WriteStorage<'a, SymbolicPoint>,
    WriteStorage<'a, Point>,
    WriteStorage<'a, PointStyle>,
    WriteStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.tool_change_event_reader = Some(world.fetch_mut::<ToolChangeEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    tool_change_event_channel,
    maybe_snap_point,
    mut mouse_event_channel,
    mut sketch_events,
    mut last_active_point_event,
    mut sym_points,
    mut points,
    mut styles,
    mut selected,
  ): Self::SystemData) {

    if let Some(reader_id) = &mut self.tool_change_event_reader {
      for ToolChangeEvent(tool) in tool_change_event_channel.read(reader_id) {
        if tool.depend_on_active_point() {
          self.mouse_event_reader = Some(mouse_event_channel.register_reader());
        } else if let Some(reader_id) = &mut self.mouse_event_reader {
            drop(reader_id);
            self.mouse_event_reader = None;
        }
      }
    }

    if let Some(reader_id) = &mut self.mouse_event_reader {
      for mouse_event in mouse_event_channel.read(reader_id) {
        match mouse_event {
          MouseEvent::MouseDown(_) => {
            if let Some(SnapPoint { position, symbo }) = maybe_snap_point.get() {

              // Get the symbolic point data from symbo
              let symbolic_point = match symbo {
                SnapPointType::NotSnapped => Some(SymbolicPoint::Free(position)),
                SnapPointType::SnapOnLine(line_ent, t) => Some(SymbolicPoint::OnLine(line_ent, t)),
                SnapPointType::SnapOnIntersection(l1_ent, l2_ent) => Some(SymbolicPoint::LineLineIntersect(l1_ent, l2_ent)),
                SnapPointType::SnapOnPoint(entity) => {

                  // If clicked on the snapped point, mark this point as last active
                  last_active_point_event.single_write(LastActivePoint::new(entity));

                  // Return none since we don't create new symbolic point
                  None
                },
              };

              // Check if we need to create a point
              if let Some(sym_point) = symbolic_point {

                let point_style = PointStyle { color: Color::red(), radius: 5. };

                // First create the entity
                let entity = entities.create();
                if let Err(err) = sym_points.insert(entity, sym_point) { panic!(err) };
                if let Err(err) = points.insert(entity, position) { panic!(err) };
                if let Err(err) = styles.insert(entity, point_style) { panic!(err) };
                if let Err(err) = selected.insert(entity, Selected) { panic!(err) };

                // Then emit an event
                sketch_events.single_write(SketchEvent::Insert(entity, Geometry::Point(sym_point, point_style)));

                // Mark this created entity as the last active point
                last_active_point_event.single_write(LastActivePoint::new(entity));
              }
            }
          },
          _ => (),
        }
      }
    }
  }
}