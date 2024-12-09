use crate::utils::read_lines;
use std::io;

pub fn checksum(fs: &Vec<Option<u32>>) -> u64 {
  let mut sum = 0;
  for i in 0..fs.len() {
    if let Some(id) = fs[i] {
      sum += i as u64 * id as u64;
    }
  }

  sum
}

pub fn read_fs(input_path: String) -> io::Result<Vec<Option<u32>>> {
  let nums: Vec<_> = read_lines(input_path)?
    .next()
    .unwrap()
    .chars()
    .map(|char| char.to_digit(10))
    .flatten()
    .collect();

  let mut fs: Vec<Option<u32>> = Vec::new();
  for i in 0..nums.len() {
    if i % 2 == 0 {
      for _j in 0..nums[i] {
        fs.push(Some(i as u32 / 2));
      }
    } else {
      for _j in 0..nums[i] {
        fs.push(None);
      }
    }
  }

  Ok(fs)
}
