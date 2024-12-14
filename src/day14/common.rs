use regex::Regex;

pub const WIDTH: i32 = 101;
pub const HEIGHT: i32 = 103;

pub struct Robot {
  pub pos: (i32, i32),
  pub velocity: (i32, i32),
}

impl Robot {
  pub fn parse(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let regex = Regex::new("-?\\d+")?;
    let nums: Vec<i32> = regex
      .find_iter(data)
      .map(|m| m.as_str().parse())
      .flatten()
      .collect();

    Ok(Self {
      pos: (nums[0], nums[1]),
      velocity: (nums[2], nums[3]),
    })
  }

  pub fn move_next(&mut self) {
    self.pos.0 = (self.pos.0 + self.velocity.0) % WIDTH;
    self.pos.1 = (self.pos.1 + self.velocity.1) % HEIGHT;

    if self.pos.0 < 0 {
      self.pos.0 = WIDTH + self.pos.0;
    }

    if self.pos.1 < 0 {
      self.pos.1 = HEIGHT + self.pos.1;
    }
  }
}

pub fn compute_quadrant_counts(robots: &Vec<Robot>) -> [i32; 4] {
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

  quadrants
}
