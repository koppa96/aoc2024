use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut count = 0;
  for line in read_lines(input_path)? {
    let mut values: Vec<_> = line
      .split(" ")
      .map(|part| part.parse::<i32>())
      .flatten()
      .collect();

    if is_safe(&values) {
      count += 1;
    } else {
      values.reverse();
      if is_safe(&values) {
        count += 1;
      }
    }
  }

  println!("{count}");

  Ok(())
}

fn is_safe(values: &Vec<i32>) -> bool {
  let increasing = values[1] > values[0];
  let mut first_error = None;

  for i in 1..values.len() {
    let current = values[i];
    let previous = match first_error {
      None => values[i - 1],
      Some(idx) => {
        if idx == i - 1 {
          values[i - 2]
        } else {
          values[i - 1]
        }
      }
    };

    let diff = (current - previous).abs();
    if diff == 0 || diff > 3 {
      if first_error.is_some() {
        return false;
      }

      first_error = Some(i);
    }

    if increasing && current - previous < 0 {
      if first_error.is_some() {
        return false;
      }

      first_error = Some(i);
    } else if !increasing && current - previous > 0 {
      if first_error.is_some() {
        return false;
      }

      first_error = Some(i);
    }
  }

  true
}
