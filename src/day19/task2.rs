use crate::utils;
use crate::utils::read_lines;
use std::collections::HashMap;

pub fn solve(input_path: String) -> utils::Result {
  let mut lines = read_lines(input_path)?;
  let first_line = lines.next().ok_or("Unexpected end of file")?;
  let mut available: Vec<_> = first_line.split(", ").collect();
  available.sort_by(|a, b| b.len().cmp(&a.len()));

  // Read the empty line
  lines.next();

  let mut count = 0;
  for line in lines {
    let mut cache = HashMap::new();
    let prefiltered_parts = &available
      .iter()
      .filter(|part| line.contains(*part))
      .map(|part| *part)
      .collect();
    count += get_composition_count(&line, &prefiltered_parts, &mut cache);
  }

  println!("{count}");

  Ok(())
}

fn get_composition_count<'a>(
  pattern: &'a str,
  available_parts: &Vec<&str>,
  cache: &mut HashMap<&'a str, usize>,
) -> usize {
  let mut count = 0;
  for part in available_parts {
    if *part == pattern {
      count += 1;
    }

    if pattern.starts_with(part) {
      let without_prefix = &pattern[part.len()..pattern.len()];
      if let Some(value) = cache.get(without_prefix) {
        count += *value;
      } else {
        let value = get_composition_count(without_prefix, available_parts, cache);
        cache.insert(without_prefix, value);
        count += value;
      }
    }
  }

  count
}
