use specs::prelude::*;
use crate::{
  resources::{
    DependencyGraph,
    events::{GeometryAction, GeometryActionChannel, GeometryActionReader},
  },
  components::{SymbolicLine, SymbolicPoint, Selected},
};

pub struct DrawParallelOnSelected {
  geometry_action_reader: Option<GeometryActionReader>,
}

impl Default for DrawParallelOnSelected {
  fn default() -> Self {
    Self { geometry_action_reader: None }
  }
}

impl<'a> System<'a> for DrawParallelOnSelected {
  type SystemData = (
    Entities<'a>,
    Read<'a, DependencyGraph>,
    Write<'a, GeometryActionChannel>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, Selected>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_action_reader = Some(world.fetch_mut::<GeometryActionChannel>().register_reader());
  }

  fn run(&mut self, (
    entities,
    dependency_graph,
    mut geometry_action_channel,
    sym_points,
    sym_lines,
    selected,
  ): Self::SystemData) {
    let mut to_insert = vec![];

    if let Some(reader_id) = &mut self.geometry_action_reader {
      for event in geometry_action_channel.read(reader_id) {
        match event {
          GeometryAction::DrawParallelOnSelected => {

            // Main functionality starts
            let mut maybe_line_ent = None;
            for (entity, _, _) in (&entities, &sym_lines, &selected).join() {
              if maybe_line_ent.is_none() {
                maybe_line_ent = Some(entity);
              } else {
                return; // Early terminate since we don't accept more than one line being selected
              }
            }

            if let Some(line_ent) = maybe_line_ent {
              for (p_ent, sym_point, _) in (&entities, &sym_points, &selected).join() {
                if !is_on_line(&p_ent, &sym_point, &line_ent, &dependency_graph) {
                  let sym_line = SymbolicLine::Parallel(line_ent, p_ent);
                  to_insert.push(sym_line);
                }
              }
            }

            break;
          },
          _ => (),
        }
      }
    }

    for sym_line in to_insert {
      geometry_action_channel.single_write(GeometryAction::InsertLine(sym_line));
    }
  }
}

fn is_on_line(p_ent: &Entity, sym_point: &SymbolicPoint, line_ent: &Entity, dependency_graph: &DependencyGraph) -> bool {
  match sym_point {
    SymbolicPoint::Free(_) | SymbolicPoint::MidPoint(_, _) => (),
    SymbolicPoint::OnLine(l, _) => {
      if *l == *line_ent {
        return true;
      }
    },
    SymbolicPoint::LineLineIntersect(l1, l2) => {
      if *l1 == *line_ent || *l2 == *line_ent {
        return true;
      }
    },
    SymbolicPoint::OnCircle(_, _) => (),
    SymbolicPoint::CircleLineIntersect(_, l, _) => {
      if *l == *line_ent {
        return true;
      }
    }
  }
  if let Some(dependents) = dependency_graph.get_direct_dependents(p_ent) {
    return dependents.contains(line_ent);
  }
  return false;
}