#![feature(type_alias_enum_variants)]
#![feature(duration_float)]

extern crate piston_window;
extern crate specs;

#[macro_use] mod util;
mod components;
mod resources;
mod systems;

use piston_window::{PistonWindow, WindowSettings};
use specs::prelude::*;
use util::{Color, Vector2};
use components::*;
use resources::*;
use systems::*;

fn main() {

  // Create a world
  let mut world = World::new();

  // Create a window
  let window : PistonWindow = WindowSettings::new("Geometry Sketchpad - Untitled.gsp", WINDOW_SIZE).build().unwrap();
  let window_system = WindowSystem { window };

  // Create dispatcher
  let mut dispatcher = DispatcherBuilder::new()
    .with(ExitSystem, "exit_system", &[])
    .with(ViewportSystem, "viewport_system", &[])
    .with(SpatialHashCache, "spatial_hash_cache", &["viewport_system"])
    .with(ChangeToolSystem, "change_tool_system", &[])
    .with(SnapPointSystem, "snap_point_system", &["spatial_hash_cache", "change_tool_system"])
    .with(SnapPointRenderer::default(), "snap_point_renderer", &["snap_point_system"])
    .with(CreatePointSystem, "create_point_system", &["snap_point_system"])
    .with(SolverSystem, "solver_system", &[]) // TODO: Make sure you understand why solver_system cannot depend on create_point_system
    .with_thread_local(window_system)
    .build();

  // Setup resources
  dispatcher.setup(&mut world);

  // ============ TEMP START ============
  let point_style = PointStyle { color: Color::red(), radius: 5. };
  let line_style = LineStyle { color: Color::blue(), width: 2. };

  let pa = world.create_entity().with(SymbolicPoint::Free(vec2![-4., 0.])).with(point_style).build();
  let pb = world.create_entity().with(SymbolicPoint::Free(vec2![-2., 2.])).with(point_style).build();
  let pc = world.create_entity().with(SymbolicPoint::Free(vec2![2., 2.])).with(point_style).build();
  let pd = world.create_entity().with(SymbolicPoint::Free(vec2![4., 0.])).with(point_style).build();

  let _lx = world.create_entity().with(SymbolicLine::TwoPoints(pa, pd)).with(line_style).with(Selected).build();
  let _l1 = world.create_entity().with(SymbolicLine::TwoPoints(pa, pb)).with(line_style).build();
  let _l2 = world.create_entity().with(SymbolicLine::TwoPoints(pc, pd)).with(line_style).build();

  // let pe = world.create_entity().with(SymbolicPoint::LineLineIntersect(l1, l2)).with(point_style).build();
  // let pf = world.create_entity().with(SymbolicPoint::OnLine(lx, 3.)).with(point_style).build();

  // let _l3 = world.create_entity().with(SymbolicLine::TwoPoints(pe, pf)).with(line_style).build();
  // let _l4 = world.create_entity().with(SymbolicLine::Parallel(lx, pe)).with(line_style).build();
  // ============ TEMP END ============

  // Enter game main loop
  while world.fetch::<FinishState>().not_finished() {
    dispatcher.dispatch(&mut world);
  }
}