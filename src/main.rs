#![feature(type_alias_enum_variants)]
#![feature(duration_float)]

extern crate piston_window;
extern crate specs;

#[macro_use] mod math;
mod systems;
mod resources;
mod components;
mod util;

use piston_window::{PistonWindow, WindowSettings};
use specs::prelude::*;
use resources::{FinishState, Viewport, WINDOW_SIZE, InputState, ToolState, DeltaTime};
use systems::{ViewportSystem, WindowSystem, CreatePointSystem, SelectPointSystem, SolverSystem};
use components::{
  point::*,
  line::*,
};
use util::Color;
use math::Vector2;

fn main() {

  // Create a world
  let mut world = World::new();

  // Insert resources
  world.insert(FinishState::default());
  world.insert(Viewport::default());
  world.insert(DeltaTime::default());
  world.insert(InputState::default());
  world.insert(ToolState::default());

  // Create a window
  let window : PistonWindow = WindowSettings::new("Geometry Sketchpad - Untitled.gsp", WINDOW_SIZE).build().unwrap();
  let window_system = WindowSystem { window };

  // Create dispatcher
  let mut dispatcher = DispatcherBuilder::new()
    .with(ViewportSystem, "viewport", &[])
    .with(CreatePointSystem::default(), "create_point", &[])
    .with(SelectPointSystem, "select_point", &[])
    .with(SolverSystem, "solver", &[])
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

  let lx = world.create_entity().with(SymbolicLine::TwoPoints(pa, pd)).with(line_style).build();
  let l1 = world.create_entity().with(SymbolicLine::TwoPoints(pa, pb)).with(line_style).build();
  let l2 = world.create_entity().with(SymbolicLine::TwoPoints(pc, pd)).with(line_style).build();

  // let pe = world.create_entity().with(SymbolicPoint::LineLineIntersect(l1, l2)).with(point_style).build();
  // let pf = world.create_entity().with(SymbolicPoint::OnLine(lx, 3.)).with(point_style).build();

  // let _l3 = world.create_entity().with(SymbolicLine::TwoPoints(pe, pf)).with(line_style).build();
  // ============ TEMP END ============

  // Enter game main loop
  while world.fetch::<FinishState>().not_finished() {
    dispatcher.dispatch(&mut world);
  }
}