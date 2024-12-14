use crate::day14::common::{Robot, HEIGHT, WIDTH};
use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut robots: Vec<_> = read_lines(input_path)?
    .map(|line| Robot::parse(&line))
    .flatten()
    .collect();

  for i in 0..10000 {
    for robot in &mut robots {
      robot.move_next();
    }

    if is_potential_xmas_tree(&robots) {
      println!("{}", i + 1);
      print(&robots);
    }
  }

  Ok(())
}

fn is_potential_xmas_tree(robots: &Vec<Robot>) -> bool {
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

  // If the robots are very grouped up in one quadrant, it's sus
  quadrants[0] > (quadrants[1] + quadrants[2] + quadrants[3])
    || quadrants[1] > (quadrants[0] + quadrants[2] + quadrants[3])
    || quadrants[2] > (quadrants[0] + quadrants[1] + quadrants[3])
    || quadrants[3] > (quadrants[0] + quadrants[1] + quadrants[2])
}

fn print(robots: &Vec<Robot>) {
  for i in 0..HEIGHT {
    for j in 0..WIDTH {
      if has_robot((j, i), robots) {
        print!("█");
      } else {
        print!(" ");
      }
    }
    println!();
  }

  println!();
}

fn has_robot(pos: (i32, i32), robots: &Vec<Robot>) -> bool {
  for robot in robots {
    if robot.pos == pos {
      return true;
    }
  }

  false
}
