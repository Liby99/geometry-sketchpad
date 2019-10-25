use std::collections::HashSet;
use specs::prelude::*;
use crate::{math::*, events::*, utilities::*, resources::*, components::{symbolics::*, virtual_shapes::*}};

pub struct VirtualShapeSolver {
  geometry_event_reader: Option<GeometryEventReader>,
}

impl Default for VirtualShapeSolver {
  fn default() -> Self {
    Self { geometry_event_reader: None }
  }
}

struct ToCompute(Entity, GeometrySymbol);

enum SolveResult {
  AlreadyComputed, // Already Computed
  SolvedPoint(VirtualPoint), // The result of point
  SolvedLine(VirtualLine), // The result of line
  SolvedCircle(VirtualCircle), // The result of circle
  Request(Entity), // Need other dependency
  Undefined, // The result does not exist
}

impl<'a> System<'a> for VirtualShapeSolver {
  type SystemData = (
    Read<'a, GeometryEventChannel>,
    Read<'a, DependencyGraph>,
    ReadStorage<'a, SymbolicPoint>,
    ReadStorage<'a, SymbolicLine>,
    ReadStorage<'a, SymbolicCircle>,
    WriteStorage<'a, VirtualPoint>,
    WriteStorage<'a, VirtualLine>,
    WriteStorage<'a, VirtualCircle>,
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.geometry_event_reader = Some(world.fetch_mut::<GeometryEventChannel>().register_reader());
  }

  fn run(&mut self, (
    geometry_event_channel,
    dependency_graph,
    sym_points,
    sym_lines,
    sym_circles,
    mut virt_points,
    mut virt_lines,
    mut virt_circles,
  ): Self::SystemData) {
    let mut to_process = Vec::new();
    let mut cannot_compute = HashSet::new();

    // First get all the things to process
    if let Some(reader) = &mut self.geometry_event_reader {
      for event in geometry_event_channel.read(reader) {
        match event {
          GeometryEvent::Inserted(ent, geom, _) => {
            to_process.push(ToCompute(*ent, geom.clone().into()));
          },
          GeometryEvent::Removed(_, _, _) => (),
          GeometryEvent::PointUpdated(ent, _, _, _) => {
            for dep in dependency_graph.get_all_dependents(ent) {
              to_process.push(ToCompute(dep, get_symbol(dep, &sym_points, &sym_lines, &sym_circles)));
            }
          },
          GeometryEvent::PointUpdateFinished(_, _, _, _) => (),
        }
      }
    }

    // Then remove them from computed
    for elem in &to_process {
      match elem {
        ToCompute(ent, GeometrySymbol::Point(_)) => { virt_points.remove(*ent); }
        ToCompute(ent, GeometrySymbol::Line(_)) => { virt_lines.remove(*ent); }
        ToCompute(ent, GeometrySymbol::Circle(_)) => { virt_circles.remove(*ent); }
      }
    }

    // Finally process them in sequence
    while !to_process.is_empty() {
      let to_comp = to_process.pop().unwrap(); // We can do this because we have checked it is not empty
      let ent = to_comp.0.clone();
      let sym = to_comp.1.clone();
      match solve(ent, sym, &virt_points, &virt_lines, &virt_circles) {
        SolveResult::AlreadyComputed => (),
        SolveResult::Undefined => { cannot_compute.insert(ent); },
        SolveResult::SolvedPoint(vp) => if let Err(err) = virt_points.insert(ent, vp) { panic!(err) },
        SolveResult::SolvedLine(vl) => if let Err(err) = virt_lines.insert(ent, vl) { panic!(err) },
        SolveResult::SolvedCircle(vc) => if let Err(err) = virt_circles.insert(ent, vc) { panic!(err) },
        SolveResult::Request(req_ent) => {
          if !cannot_compute.contains(&req_ent) {
            to_process.push(to_comp);
            to_process.push(ToCompute(req_ent, get_symbol(req_ent, &sym_points, &sym_lines, &sym_circles)));
          }
        }
      }
    }
  }
}

fn get_symbol<'a>(
  ent: Entity,
  sym_points: &ReadStorage<'a, SymbolicPoint>,
  sym_lines: &ReadStorage<'a, SymbolicLine>,
  sym_circles: &ReadStorage<'a, SymbolicCircle>,
) -> GeometrySymbol {
  if let Some(sym_point) = sym_points.get(ent) {
    GeometrySymbol::Point(*sym_point)
  } else if let Some(sym_line) = sym_lines.get(ent) {
    GeometrySymbol::Line(*sym_line)
  } else if let Some(sym_circle) = sym_circles.get(ent) {
    GeometrySymbol::Circle(*sym_circle)
  } else {
    panic!("Cannot find symbol");
  }
}

fn solve<'a>(
  ent: Entity,
  sym: GeometrySymbol,
  virt_points: &WriteStorage<'a, VirtualPoint>,
  virt_lines: &WriteStorage<'a, VirtualLine>,
  virt_circles: &WriteStorage<'a, VirtualCircle>,
) -> SolveResult {
  match sym {
    GeometrySymbol::Point(sym_point) => solve_point(ent, sym_point, &virt_points, &virt_lines, &virt_circles),
    GeometrySymbol::Line(sym_line) => solve_line(ent, sym_line, &virt_points, &virt_lines, &virt_circles),
    GeometrySymbol::Circle(sym_circle) => solve_circle(ent, sym_circle, &virt_points, &virt_lines, &virt_circles),
  }
}

fn solve_point<'a>(
  ent: Entity,
  sym_point: SymbolicPoint,
  virt_points: &WriteStorage<'a, VirtualPoint>,
  virt_lines: &WriteStorage<'a, VirtualLine>,
  virt_circles: &WriteStorage<'a, VirtualCircle>,
) -> SolveResult {
  if virt_points.contains(ent) {
    SolveResult::AlreadyComputed
  } else {
    match sym_point {
      SymbolicPoint::Fixed(pos) => SolveResult::SolvedPoint(pos),
      SymbolicPoint::Free(pos) => SolveResult::SolvedPoint(pos),
      SymbolicPoint::MidPoint(p1_ent, p2_ent) => match virt_points.get(p1_ent) {
        Some(vp1) => match virt_points.get(p2_ent) {
          Some(vp2) => SolveResult::SolvedPoint((*vp1 + *vp2) / 2.0.into()),
          None => SolveResult::Request(p2_ent),
        },
        None => SolveResult::Request(p1_ent),
      },
      SymbolicPoint::OnLine(l_ent, t) => match virt_lines.get(l_ent) {
        Some(VirtualLine { from, to, .. }) => SolveResult::SolvedPoint(*from + (*to - *from) * t),
        None => SolveResult::Request(l_ent),
      },
      SymbolicPoint::LineLineIntersect(l1_ent, l2_ent) => match virt_lines.get(l1_ent) {
        Some(vl1) => match virt_lines.get(l2_ent) {
          Some(vl2) => match (*vl1).intersect(*vl2) {
            Some(p) => SolveResult::SolvedPoint(p),
            None => SolveResult::Undefined,
          },
          None => SolveResult::Request(l2_ent),
        },
        None => SolveResult::Request(l1_ent),
      },
      SymbolicPoint::OnCircle(c_ent, theta) => match virt_circles.get(c_ent) {
        Some(c) => SolveResult::SolvedPoint(c.center + VirtualPosition(vec2![theta.cos(), theta.sin()]) * c.radius),
        None => SolveResult::Request(c_ent),
      },
      SymbolicPoint::CircleLineIntersect(c_ent, l_ent, ity) => match virt_circles.get(c_ent) {
        Some(c) => match virt_lines.get(l_ent) {
          Some(l) => match (*c).intersect(*l) {
            VirtualCircleIntersect::TwoPoints(p1, p2) => match ity {
              CircleIntersectId::First => SolveResult::SolvedPoint(p1),
              CircleIntersectId::Second => SolveResult::SolvedPoint(p2),
            },
            VirtualCircleIntersect::OnePoint(p) => SolveResult::SolvedPoint(p),
            VirtualCircleIntersect::None => SolveResult::Undefined,
          },
          None => SolveResult::Request(l_ent),
        },
        None => SolveResult::Request(c_ent),
      },
      SymbolicPoint::CircleCircleIntersect(c1_ent, c2_ent, ity) => match virt_circles.get(c1_ent) {
        Some(c1) => match virt_circles.get(c2_ent) {
          Some(c2) => match (*c1).intersect(*c2) {
            VirtualCircleIntersect::TwoPoints(p1, p2) => match ity {
              CircleIntersectId::First => SolveResult::SolvedPoint(p1),
              CircleIntersectId::Second => SolveResult::SolvedPoint(p2),
            },
            VirtualCircleIntersect::OnePoint(p) => SolveResult::SolvedPoint(p),
            VirtualCircleIntersect::None => SolveResult::Undefined,
          },
          None => SolveResult::Request(c2_ent),
        },
        None => SolveResult::Request(c1_ent),
      }
    }
  }
}

fn solve_line<'a>(
  ent: Entity,
  sym_line: SymbolicLine,
  virt_points: &WriteStorage<'a, VirtualPoint>,
  virt_lines: &WriteStorage<'a, VirtualLine>,
  _virt_circles: &WriteStorage<'a, VirtualCircle>, // Don't need circle for now
) -> SolveResult {
  if virt_lines.contains(ent) {
    SolveResult::AlreadyComputed
  } else {
    match sym_line {
      SymbolicLine::Straight(p1_ent, p2_ent) => match virt_points.get(p1_ent) {
        Some(p1) => match virt_points.get(p2_ent) {
          Some(p2) => SolveResult::SolvedLine(VirtualLine { from: *p1, to: *p2, line_type: LineType::Straight }),
          None => SolveResult::Request(p2_ent),
        },
        None => SolveResult::Request(p1_ent),
      },
      SymbolicLine::Ray(p1_ent, p2_ent) => match virt_points.get(p1_ent) {
        Some(p1) => match virt_points.get(p2_ent) {
          Some(p2) => SolveResult::SolvedLine(VirtualLine { from: *p1, to: *p2, line_type: LineType::Ray }),
          None => SolveResult::Request(p2_ent),
        },
        None => SolveResult::Request(p1_ent),
      },
      SymbolicLine::Segment(p1_ent, p2_ent) => match virt_points.get(p1_ent) {
        Some(p1) => match virt_points.get(p2_ent) {
          Some(p2) => SolveResult::SolvedLine(VirtualLine { from: *p1, to: *p2, line_type: LineType::Segment }),
          None => SolveResult::Request(p2_ent),
        },
        None => SolveResult::Request(p1_ent),
      },
      SymbolicLine::Parallel(l_ent, p_ent) => match virt_lines.get(l_ent) {
        Some(l) => match virt_points.get(p_ent) {
          Some(p) => SolveResult::SolvedLine(VirtualLine { from: *p, to: *p + (l.to - l.from), line_type: LineType::Straight }),
          None => SolveResult::Request(p_ent),
        },
        None => SolveResult::Request(l_ent),
      },
      SymbolicLine::Perpendicular(l_ent, p_ent) => match virt_lines.get(l_ent) {
        Some(l) => match virt_points.get(p_ent) {
          Some(p) => {
            let dir : Vector2 = (l.to - l.from).into();
            let perp_dir : Vector2 = vec2![-dir.y, dir.x];
            SolveResult::SolvedLine(VirtualLine { from: *p, to: *p + perp_dir.into(), line_type: LineType::Straight })
          },
          None => SolveResult::Request(p_ent),
        },
        None => SolveResult::Request(l_ent),
      },
    }
  }
}

fn solve_circle<'a>(
  ent: Entity,
  sym_circle: SymbolicCircle,
  virt_points: &WriteStorage<'a, VirtualPoint>,
  _virt_lines: &WriteStorage<'a, VirtualLine>, // Don't need virtual lines for now
  virt_circles: &WriteStorage<'a, VirtualCircle>,
) -> SolveResult {
  if virt_circles.contains(ent) {
    SolveResult::AlreadyComputed
  } else {
    match sym_circle {
      SymbolicCircle::CenterRadius(p1_ent, p2_ent) => match virt_points.get(p1_ent) {
        Some(p1) => match virt_points.get(p2_ent) {
          Some(p2) => SolveResult::SolvedCircle(VirtualCircle { center: *p1, radius: (*p2 - *p1).magnitude() }),
          None => SolveResult::Request(p2_ent),
        },
        None => SolveResult::Request(p1_ent),
      }
    }
  }
}