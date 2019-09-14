extern crate piston_window;
extern crate specs;

mod systems;
mod states;
mod components;
#[macro_use] mod math;
mod util;

use piston_window::{PistonWindow, WindowSettings};
use specs::prelude::*;
use states::FinishState;
use systems::{RenderSystem, SolverSystem};
use components::{
  point::*,
  line::*,
};
use util::Color;
use math::Vector2;

fn main() {

  // Create a world
  let mut world = World::new();
  world.insert(FinishState(false));
  world.register::<Point>();
  world.register::<SymbolicPoint>();
  world.register::<PointStyle>();
  world.register::<Line>();
  world.register::<SymbolicLine>();
  world.register::<LineStyle>();

  // ============ TEMP START ============
  let point_style = PointStyle { color: Color::red(), radius: 5. };
  let line_style = LineStyle { color: Color::blue(), width: 2. };

  let pa = world.create_entity().with(SymbolicPoint::Free(vec2![-40., 0.])).with(point_style).build();
  let pb = world.create_entity().with(SymbolicPoint::Free(vec2![-20., 20.])).with(point_style).build();
  let pc = world.create_entity().with(SymbolicPoint::Free(vec2![20., 20.])).with(point_style).build();
  let pd = world.create_entity().with(SymbolicPoint::Free(vec2![40., 0.])).with(point_style).build();

  let lx = world.create_entity().with(SymbolicLine::TwoPoints(pa, pd)).with(line_style).build();
  let l1 = world.create_entity().with(SymbolicLine::TwoPoints(pa, pb)).with(line_style).build();
  let l2 = world.create_entity().with(SymbolicLine::TwoPoints(pc, pd)).with(line_style).build();

  let pe = world.create_entity().with(SymbolicPoint::LineLineIntersect(l1, l2)).with(point_style).build();
  let pf = world.create_entity().with(SymbolicPoint::OnLine(lx, 15.)).with(point_style).build();

  let _l3 = world.create_entity().with(SymbolicLine::TwoPoints(pe, pf)).with(line_style).build();
  // ============ TEMP END ============

  // Create a window
  let window : PistonWindow = WindowSettings::new("Geometry Sketchpad - Untitled.gsp", [960, 720]).build().unwrap();
  let render_system = RenderSystem { window };

  // Create dispatcher
  let mut dispatcher = DispatcherBuilder::new()
    .with(SolverSystem, "solver", &[])
    .with_thread_local(render_system)
    .build();

  // Enter game main loop
  while world.fetch::<FinishState>().not_finished() {
    dispatcher.dispatch(&mut world);
  }
}