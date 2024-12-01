use crate::day1::common::read_input;
use crate::utils;

pub fn solve(input_path: String) -> utils::Result {
  let mut input = read_input(input_path)?;

  input.list1.sort();
  input.list2.sort();

  let mut sum = 0;
  for i in 0..input.list1.len() {
    sum += (input.list1[i] - input.list2[i]).abs();
  }

  println!("{sum}");

  Ok(())
}
