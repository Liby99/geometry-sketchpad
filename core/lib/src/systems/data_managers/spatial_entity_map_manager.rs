use specs::prelude::*;
use crate::{
  math::*,
  events::*,
  resources::*,
  utilities::*,
  components::{
    screen_shapes::*,
    markers::*,
  },
};

pub struct SpatialEntityMapManager {
  geometry_event_reader: Option<GeometryEventReader>,
  viewport_event_reader: Option<ViewportEventReader>,
  marker_event_reader: Option<MarkerEventReader>,
}

impl Default for SpatialEntityMapManager {
  fn default() -> Self {
    Self {
      geometry_event_reader: None,
      viewport_event_reader: None,
      marker_event_reader: None,
    }
  }
}

impl<'a> System<'a> for SpatialEntityMapManager {
  type SystemData = (
    Entities<'a>,
    Read<'a, GeometryEventChannel>,
    Read<'a, ViewportEventChannel>,
    Read<'a, MarkerEventChannel>,
    Write<'a, SpatialEntityMap>,
    ReadStorage<'a, ScreenPosition>,
    ReadStorage<'a, ScreenLine>,
    ReadStorage<'a, ScreenCircle>,
    ReadStorage<'a, Hidden>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_event_reader = Some(world.fetch_mut::<GeometryEventChannel>().register_reader());
    self.viewport_event_reader = Some(world.fetch_mut::<ViewportEventChannel>().register_reader());
    self.marker_event_reader = Some(world.fetch_mut::<MarkerEventChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    geometry_event_channel,
    viewport_event_channel,
    marker_event_channel,
    mut spatial_entity_map,
    screen_points,
    screen_lines,
    screen_circles,
    hiddens,
  ): Self::SystemData) {

    // First check if we need to update all. It happens when a viewport event happens
    if let Some(reader) = &mut self.viewport_event_reader {
      let mut need_add_all = false;
      for event in viewport_event_channel.read(reader) {
        match event {
          ViewportEvent::Move(_) => spatial_entity_map.clear(),
          ViewportEvent::Scale(Vector2 { x, y }) => spatial_entity_map.set_size(*x, *y),
        }
        need_add_all = true;
      }
      if need_add_all {
        for (ent, screen_point, _) in (&entities, &screen_points, !&hiddens).join() {
          spatial_entity_map.insert_point(ent, (*screen_point).into());
        }
        for (ent, screen_line, _) in (&entities, &screen_lines, !&hiddens).join() {
          spatial_entity_map.insert_line(ent, (*screen_line).into());
        }
        for (ent, screen_circle, _) in (&entities, &screen_circles, !&hiddens).join() {
          spatial_entity_map.insert_circle(ent, (*screen_circle).into());
        }
      }
    }

    // Then read the geometry events to determine which ones to add
    if let Some(reader) = &mut self.geometry_event_reader {
      for event in geometry_event_channel.read(reader) {
        match event {
          GeometryEvent::Inserted(ent, geom, _) => {
            insert(ent, geom, &mut spatial_entity_map, &screen_points, &screen_lines, &screen_circles);
          },
          GeometryEvent::Removed(ent, _, _) => {
            spatial_entity_map.remove_from_all(*ent);
          },
          GeometryEvent::Updated(ent, _, geom, _) => {
            spatial_entity_map.remove_from_all(*ent);
            insert(ent, geom, &mut spatial_entity_map, &screen_points, &screen_lines, &screen_circles);
          },
          _ => (),
        }
      }
    }

    // Finally handle the marker events. When hiding/unhiding, should add/remove respectively
    if let Some(reader) = &mut self.marker_event_reader {
      for event in marker_event_channel.read(reader) {
        match event {
          MarkerEvent::Hide(ent, _) => {
            spatial_entity_map.remove_from_all(*ent);
          },
          MarkerEvent::Unhide(ent, _) => {
            insert_without_geom(ent, &mut spatial_entity_map, &screen_points, &screen_lines, &screen_circles);
          },
          _ => (), // Do nothing otherwise
        }
      }
    }
  }
}

fn insert<'a>(
  ent: &Entity,
  geom: &Geometry,
  spatial_entity_map: &mut SpatialEntityMap,
  screen_points: &ReadStorage<'a, ScreenPoint>,
  screen_lines: &ReadStorage<'a, ScreenLine>,
  screen_circles: &ReadStorage<'a, ScreenCircle>,
) {
  match geom {
    Geometry::Point(_, _) => if let Some(screen_point) = screen_points.get(*ent) {
      spatial_entity_map.insert_point(*ent, (*screen_point).into());
    },
    Geometry::Line(_, _) => if let Some(screen_line) = screen_lines.get(*ent) {
      spatial_entity_map.insert_line(*ent, (*screen_line).into());
    },
    Geometry::Circle(_, _) => if let Some(screen_circle) = screen_circles.get(*ent) {
      spatial_entity_map.insert_circle(*ent, (*screen_circle).into());
    },
  }
}

fn insert_without_geom<'a>(
  ent: &Entity,
  spatial_entity_map: &mut SpatialEntityMap,
  screen_points: &ReadStorage<'a, ScreenPoint>,
  screen_lines: &ReadStorage<'a, ScreenLine>,
  screen_circles: &ReadStorage<'a, ScreenCircle>,
) {
  if let Some(screen_point) = screen_points.get(*ent) {
    spatial_entity_map.insert_point(*ent, (*screen_point).into());
  } else if let Some(screen_line) = screen_lines.get(*ent) {
    spatial_entity_map.insert_line(*ent, (*screen_line).into());
  } else if let Some(screen_circle) = screen_circles.get(*ent) {
    spatial_entity_map.insert_circle(*ent, (*screen_circle).into());
  }
}