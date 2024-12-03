use crate::utils;
use crate::utils::read_lines;
use regex::Regex;

pub fn solve(input_path: String) -> utils::Result {
  let regex = Regex::new("mul\\((\\d+),(\\d+)\\)")?;

  let mut sum = 0;
  for line in read_lines(input_path)? {
    for capture in regex.captures_iter(&line) {
      let first: i32 = capture
        .get(1)
        .ok_or("failed to get the first item in the capture group")?
        .as_str()
        .parse()?;

      let second: i32 = capture
        .get(2)
        .ok_or("failed to get the second item in the capture group")?
        .as_str()
        .parse()?;

      sum += first * second;
    }
  }

  println!("{sum}");

  Ok(())
}
