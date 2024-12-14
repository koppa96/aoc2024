use crate::day14::common::{Robot, HEIGHT, WIDTH};
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
  let mut quadrants: [i32; 4] = [0, 0, 0, 0];
  for robot in robots {
    if robot.pos.0 < WIDTH / 2 && robot.pos.1 < HEIGHT / 2 {
      quadrants[0] += 1;
    } else if robot.pos.0 > WIDTH / 2 && robot.pos.1 < HEIGHT / 2 {
      quadrants[1] += 1;
    } else if robot.pos.0 < WIDTH / 2 && robot.pos.1 > HEIGHT / 2 {
      quadrants[2] += 1;
    } else if robot.pos.0 > WIDTH / 2 && robot.pos.1 > HEIGHT / 2 {
      quadrants[3] += 1;
    }
  }

  quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}
