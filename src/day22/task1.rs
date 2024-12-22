use crate::day22::common::next;
use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut sum = 0;
  for line in read_lines(input_path)? {
    let mut num: usize = line.parse()?;
    for _ in 0..2000 {
      num = next(num);
    }

    sum += num;
  }

  println!("{sum}");

  Ok(())
}
