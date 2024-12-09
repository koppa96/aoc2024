use crate::utils::read_lines;
use std::io;

pub fn read_nums(input_path: String) -> io::Result<Vec<u32>> {
  Ok(
    read_lines(input_path)?
      .next()
      .unwrap()
      .chars()
      .map(|char| char.to_digit(10))
      .flatten()
      .collect(),
  )
}
