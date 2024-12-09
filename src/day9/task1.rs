use crate::day9::common::read_nums;
use crate::utils;
use std::io;

pub fn solve(input_path: String) -> utils::Result {
  let mut blocks = read_blocks(input_path)?;
  compact(&mut blocks);
  println!("{}", checksum(&blocks));

  Ok(())
}

fn read_blocks(input_path: String) -> io::Result<Vec<Option<u32>>> {
  let nums: Vec<_> = read_nums(input_path)?;
  let mut blocks: Vec<Option<u32>> = Vec::new();
  for i in 0..nums.len() {
    if i % 2 == 0 {
      for _j in 0..nums[i] {
        blocks.push(Some(i as u32 / 2));
      }
    } else {
      for _j in 0..nums[i] {
        blocks.push(None);
      }
    }
  }

  Ok(blocks)
}

fn compact(blocks: &mut Vec<Option<u32>>) {
  let mut i = 0;
  let mut j = blocks.len() - 1;

  while i < j {
    while i < j && blocks[i] != None {
      i += 1;
    }

    while i < j && blocks[j] == None {
      j -= 1;
    }

    if i >= j {
      break;
    }

    blocks[i] = blocks[j];
    blocks[j] = None;
  }
}

fn checksum(fs: &Vec<Option<u32>>) -> usize {
  let mut sum = 0;
  for i in 0..fs.len() {
    if let Some(id) = fs[i] {
      sum += i * id as usize;
    }
  }

  sum
}
