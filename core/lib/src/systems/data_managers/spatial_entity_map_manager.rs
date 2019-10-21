use specs::prelude::*;
use crate::{events::*, resources::*, utilities::*, components::screen_shapes::*};

pub struct SpatialEntityMapManager {
  geometry_event_reader: Option<GeometryEventReader>,
}

impl Default for SpatialEntityMapManager {
  fn default() -> Self {
    Self { geometry_event_reader: None }
  }
}

impl<'a> System<'a> for SpatialEntityMapManager {
  type SystemData = (
    Read<'a, GeometryEventChannel>,
    Write<'a, SpatialEntityMap>,
    ReadStorage<'a, ScreenPosition>,
    ReadStorage<'a, ScreenLine>,
    ReadStorage<'a, ScreenCircle>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_event_reader = Some(world.fetch_mut::<GeometryEventChannel>().register_reader());
  }

  fn run(&mut self, (
    geometry_event_channel,
    mut spatial_entity_map,
    screen_points,
    screen_lines,
    screen_circles,
  ): Self::SystemData) {
    if let Some(reader) = &mut self.geometry_event_reader {
      for event in geometry_event_channel.read(reader) {
        match event {
          GeometryEvent::Inserted(ent, geom) => {
            insert(ent, geom, &mut spatial_entity_map, &screen_points, &screen_lines, &screen_circles);
          },
          GeometryEvent::Removed(ent, _) => {
            spatial_entity_map.remove_from_all(*ent);
          },
          GeometryEvent::Updated(ent, _, geom) => {
            spatial_entity_map.remove_from_all(*ent);
            insert(ent, geom, &mut spatial_entity_map, &screen_points, &screen_lines, &screen_circles);
          },
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
    Geometry::Point(_, _) => {
      if let Some(screen_point) = screen_points.get(*ent) {
        spatial_entity_map.insert_point(*ent, (*screen_point).into());
      }
    },
    Geometry::Line(_, _) => {
      if let Some(screen_line) = screen_lines.get(*ent) {
        spatial_entity_map.insert_line(*ent, (*screen_line).into());
      }
    },
    Geometry::Circle(_, _) => {
      if let Some(screen_circle) = screen_circles.get(*ent) {
        spatial_entity_map.insert_circle(*ent, (*screen_circle).into());
      }
    },
  }
}