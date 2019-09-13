use geopad::{
  geometry::{solve, SolveError, Context, PointConstruct, LineConstruct},
  math::Vector2,
};

#[test]
fn test_lines_1() -> Result<(), SolveError> {
  let mut context = Context::new();

  //                 y
  //                 ^
  //                 |
  //   - - - - - - - E - - - - - - - -
  //               /   \
  //             B       C
  //           /           \
  //   - - - A - - - * - - - D - - - > x
  //
  // Construct points A, B, C and D
  // Construct line AB and line CD
  // AB and CD intersects at E
  // Though E construct line_3 parallel to x

  // Add all the points given above settings
  let pa = context.add_point(PointConstruct::Free { pos: Vector2::new(-20., 0.) });
  let pb = context.add_point(PointConstruct::Free { pos: Vector2::new(-10., 10.) });
  let pc = context.add_point(PointConstruct::Free { pos: Vector2::new(10., 10.) });
  let pd = context.add_point(PointConstruct::Free { pos: Vector2::new(20., 0.) });
  let x_axis = context.add_line(LineConstruct::TwoPoint { p1: pa, p2: pd });
  let line_ab = context.add_line(LineConstruct::TwoPoint { p1: pa, p2: pb });
  let line_cd = context.add_line(LineConstruct::TwoPoint { p1: pd, p2: pc });
  let pe = context.add_point(PointConstruct::LineLineIntersect { l1: line_ab, l2: line_cd });
  let l3 = context.add_line(LineConstruct::Parallel { l: x_axis, p: pe });

  // Solve for the solutions
  let solution = solve(&context)?;

  // Test the calculations
  assert!(Some(&Vector2 { x: 0., y: 20. }) == solution.get_point(pe), "pe should have position [0, 2]");
  assert!(solution.get_line(l3).map(|l| l.direction == Vector2 { x: 1., y: 0. }).unwrap_or(false), "line 3 should have direction [1, 0]");

  Ok(())
}