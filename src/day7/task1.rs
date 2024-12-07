use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut sum = 0;
  for line in read_lines(input_path)? {
    let parts: Vec<_> = line.split(": ").collect();
    let result: u64 = parts[0].parse()?;
    let operands = parts[1]
      .split(" ")
      .map(|op| op.parse::<u64>())
      .flatten()
      .collect::<Vec<_>>();

    if is_possible(result, operands[0], &operands[1..operands.len()]) {
      sum += result;
    }
  }

  println!("{sum}");

  Ok(())
}

fn is_possible(result: u64, current: u64, operands: &[u64]) -> bool {
  if operands.len() == 0 {
    return current == result;
  }

  let first_op = operands[0];
  if current * first_op <= result {
    let multiplication_possible =
      is_possible(result, current * first_op, &operands[1..operands.len()]);
    if multiplication_possible {
      return true;
    }
  }

  if current + first_op > result {
    return false;
  }

  is_possible(result, current + first_op, &operands[1..operands.len()])
}
