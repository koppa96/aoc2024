use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut count = 0;
  for line in read_lines(input_path)? {
    if is_safe(&line) {
      count += 1;
    }
  }

  println!("{count}");

  Ok(())
}

fn is_safe(line: &str) -> bool {
  let values: Vec<_> = line
    .split(" ")
    .map(|part| part.parse::<i32>())
    .flatten()
    .collect();

  let increasing = values[1] > values[0];

  for i in 1..values.len() {
    let diff = (values[i] - values[i - 1]).abs();
    if diff == 0 || diff > 3 {
      return false;
    }

    if increasing && values[i] - values[i - 1] < 0 {
      return false;
    } else if !increasing && values[i] - values[i - 1] > 0 {
      return false;
    }
  }

  true
}
