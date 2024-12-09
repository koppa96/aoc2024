use crate::day9::common::{checksum, read_fs};
use crate::utils;

pub fn solve(input_path: String) -> utils::Result {
  let mut fs = read_fs(input_path)?;
  compact(&mut fs);
  println!("{}", checksum(&fs));

  Ok(())
}

fn compact(fs: &mut Vec<Option<u32>>) {
  let mut j = fs.len() as i32 - 1;
  while j >= 0 {
    while j >= 0 && fs[j as usize] == None {
      j -= 1;
    }

    if j < 0 {
      break;
    }

    let file_id = fs[j as usize].unwrap();
    let end_idx = j;
    while j >= 0 && fs[j as usize] != None && fs[j as usize].unwrap() == file_id {
      j -= 1;
    }
    let start_idx = j + 1;

    if j < 0 {
      break;
    }

    let size = end_idx - start_idx + 1;
    let space_start = find_empty_space_before(start_idx as usize, size as usize, fs);
    match space_start {
      Some(idx) => {
        for i in 0..(end_idx - start_idx + 1) {
          fs[idx + i as usize] = fs[start_idx as usize + i as usize];
          fs[start_idx as usize + i as usize] = None;
        }
      }
      None => {}
    }
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
