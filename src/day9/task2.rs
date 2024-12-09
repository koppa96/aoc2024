use crate::day9::common::{checksum, read_fs};
use crate::utils;

pub fn solve(input_path: String) -> utils::Result {
  let mut fs = read_fs(input_path)?;
  compact(&mut fs);
  println!("{}", checksum(&fs));

  Ok(())
}

fn compact(fs: &mut Vec<Option<u32>>) {
  let mut j = fs.len() - 1;
  loop {
    while j > 0 && fs[j] == None {
      j -= 1;
    }

    if j == 0 && fs[j] == None {
      break;
    }

    let end_idx = j;
    let start_idx = find_start(fs, end_idx);
    if start_idx == 0 {
      break;
    }

    j = start_idx - 1;

    let size = end_idx - start_idx + 1;
    if let Some(idx) = find_empty_space_before(start_idx, size, fs) {
      mv(fs, start_idx, idx, size);
    }
  }
}

fn find_start(fs: &mut Vec<Option<u32>>, end: usize) -> usize {
  for i in (0..=end).rev() {
    if fs[i] != fs[end] {
      return i + 1;
    }
  }

  0
}

fn mv(fs: &mut Vec<Option<u32>>, src_start_idx: usize, dst_start_idx: usize, size: usize) {
  for i in 0..size {
    fs[dst_start_idx + i] = fs[src_start_idx + i];
    fs[src_start_idx + i] = None;
  }
}

fn find_empty_space_before(idx: usize, size: usize, fs: &Vec<Option<u32>>) -> Option<usize> {
  let mut empty_space_size = 0;
  for i in 0..idx {
    if fs[i] == None {
      empty_space_size += 1;
      if empty_space_size == size {
        return Some(i - size + 1);
      }
    } else {
      empty_space_size = 0;
    }
  }

  None
}
