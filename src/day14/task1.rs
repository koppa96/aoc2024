use crate::day14::common::{compute_quadrant_counts, Robot};
use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut robots: Vec<_> = read_lines(input_path)?
    .map(|line| Robot::parse(&line))
    .flatten()
    .collect();

  for _i in 0..100 {
    for robot in &mut robots {
      robot.move_next();
    }
  }

  println!("{}", safety_factor(&robots));

  Ok(())
}

fn safety_factor(robots: &Vec<Robot>) -> i32 {
  let quadrants = compute_quadrant_counts(robots);
  quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}
