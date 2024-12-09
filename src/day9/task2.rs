use crate::day9::common::{checksum, read_fs};
use crate::utils;

pub fn solve(input_path: String) -> utils::Result {
  let mut fs = read_fs(input_path)?;
  compact(&mut fs);
  println!("{}", checksum(&fs));

  Ok(())
}

fn compact(fs: &mut Vec<Option<u32>>) {
  let mut spaces = find_empty_spaces(fs);
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

    let size = end_idx - start_idx + 1;
    if let Some(sp) = find_empty_space_before(start_idx, size, &mut spaces) {
      mv(fs, start_idx, sp.start, size);
      sp.len -= size;
      sp.start += size;
    }

    j = start_idx - 1;
  }
}

fn find_start(fs: &Vec<Option<u32>>, end: usize) -> usize {
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

fn find_empty_space_before(idx: usize, size: usize, spaces: &mut Vec<Space>) -> Option<&mut Space> {
  for sp in spaces {
    if sp.start > idx {
      return None;
    }

    if sp.len >= size {
      return Some(sp);
    }
  }

  None
}

struct Space {
  start: usize,
  len: usize,
}

fn find_empty_spaces(fs: &Vec<Option<u32>>) -> Vec<Space> {
  let mut spaces: Vec<Space> = Vec::new();
  let mut current: Option<Space> = None;
  for i in 0..fs.len() {
    match fs[i] {
      Some(_) => {
        if let Some(sp) = current {
          spaces.push(sp);
          current = None;
        }
      }
      None => {
        if let Some(sp) = &mut current {
          sp.len += 1;
        } else {
          current = Some(Space { start: i, len: 1 })
        }
      }
    }
  }

  spaces
}
