use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut lines = read_lines(input_path)?;
  let first_line = lines.next().ok_or("Unexpected end of file")?;
  let mut available: Vec<_> = first_line.split(", ").collect();
  available.sort_by(|a, b| b.len().cmp(&a.len()));

  // Read the empty line
  lines.next();

  let mut count = 0;
  for line in lines {
    if can_be_composed_from(&line, &available) {
      count += 1;
    }
  }

  println!("{count}");

  Ok(())
}

fn can_be_composed_from(pattern: &str, available_parts: &Vec<&str>) -> bool {
  for part in available_parts {
    if *part == pattern {
      return true;
    }

    if pattern.starts_with(part) {
      if can_be_composed_from(&pattern[part.len()..pattern.len()], available_parts) {
        return true;
      }
    }
  }

  false
}
